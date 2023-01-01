use ndarray::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResultSlice{
    #[serde(rename = "result_slice")]
    pub slice: Vec<Vec<i32>>,
    pub len: usize,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DecomposeParam{
    #[serde(rename = "query_position")]
    pub i: usize,
    #[serde(rename = "coefficient_p")]
    pub p: usize,
    #[serde(rename = "coefficient_N")]
    pub N:usize,
    #[serde(rename = "coefficient_lambda")]
    pub lambda:usize
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuerySlice{
    #[serde(rename = "query_slice")]
    pub slice:Vec<Vec<Vec<i32>>>
}

pub fn xor(a : &Array1<i32>, b: &Array1<i32>) -> Array1<i32>{
    let c =  a.iter()
        .zip(b.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
     
    c
}

pub fn xor_vec(a : &Vec<i32>, b: &Vec<i32>) -> Vec<i32>{
    let c =  a.iter()
        .zip(b.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
     
    c
}

pub fn Coefficient(p: usize, N:f32, i: u32) -> (usize,usize,usize,usize,usize){
    let n=N.log2().ceil(); //根据N生成n，n=lb|N|
    let m:f32=2.0;
    let p1:f32=(p-1) as f32;
    let q=m.powf(p1) as usize;
    let u = (m.powf((n/2.0))*m.powf((p1/2.0))).ceil();
    let u1 = u as usize;
    let v=(m.powf(n)/u).ceil() as usize;
    let o=(i % u.to_bits())as usize;
    let y=(i/u.to_bits()+1) as usize;
    (q,u1,v,o,y)
}

pub fn Coefficient_server(p: usize, N:usize ) -> (usize,usize,usize){
    let N=N as f32;
    let n=N.log2().ceil(); //根据N生成n，n=lb|N|
    let m:f32=2.0;
    let p1:f32=(p-1) as f32;
    let q=m.powf(p1) as usize;
    let u = (m.powf((n/2.0))*m.powf((p1/2.0))).ceil();
    let u1 = u as usize;
    let v=(m.powf(n)/u).ceil() as usize;
    (q,u1,v)
}

fn GenerateMatrix_rand(p: usize, q:usize)->Array2<i32>{
    let a = Array::random((p, q), Uniform::new(0, 2));
    return a;
}

fn IsOMatrix(a: &Array2<i32>)->bool{
    let a1=a.sum_axis(Axis(0));
    for i in 0..a1.len(){
        if a1[i]%2!=0{
            return false;
        }
    }
    return true;
}
fn IsEMatrix(a: &Array2<i32>)->bool{
    let a1=a.sum_axis(Axis(0));
    for i in 0..a1.len(){
        if a1[i]%2==0{
            return false;
        }
    }
    return true;
}

fn GenerateOMatrix(p:usize,q:usize)->Array2<i32>{
    let mut A1 = GenerateMatrix_rand(p, q);
    loop{
		if IsOMatrix(&A1){
			break;
		}
        A1 = GenerateMatrix_rand(p, q);
	}
    return A1
}

fn GenerateEMatrix(p:usize,q:usize)->Array2<i32>{
    let mut A1 = GenerateMatrix_rand(p, q);
    loop{
		if IsEMatrix(&A1){
			break;
		}
        A1 = GenerateMatrix_rand(p, q);
	}
    return A1
}

pub fn GenerateMatrix(p:usize, q:usize, y:usize,v:usize)->Vec<Array2<i32>>{
    let mut resMatrix = Vec::new();
    for i in 0..v{
        if i == y-1{
            resMatrix.push(GenerateEMatrix(p,q));
        }else{
            resMatrix.push(GenerateOMatrix(p,q))
        }
    }
    return resMatrix
}

pub fn Generatestring_rand(lambda: usize, v:usize, q:usize)->Array3<i32>{
    let mut rng = rand::thread_rng();
    let mut resarray = Array::zeros((v,q,lambda));
    for i in 0..v{
        for j in 0..q{
            loop{
            let mut res =0;
            for k in 0..lambda{
                resarray[[i,j,k]]=rng.gen_range(0..2);
                res=resarray[[i,j,k]] | res;
            }
            if res == 1 {
                break;
            }
            }
        }
    }
    return resarray;
}

pub fn PRNG(s:&Array3<i32>, u:usize, v:usize, q:usize, lambda: usize)->Array3<i32>{
    let mut resarray = Array::zeros((v,q,u));
    let n=u/lambda;
    for i in 0..v{
        for j in 0..q{
            for k in 1..(n+1){
                for m in (k-1)*lambda..k*lambda{
                    resarray[[i,j,m]]=s[[i,j,m-(k-1)*lambda]]
                }
            }
        }
    }
    return resarray;
}

pub fn Generatecw(s:&Array3<i32>,u:usize,q:usize,o:usize,y:usize)->Vec<Vec<i32>>{
    let mut A = Array::random((q, u), Uniform::new(0, 2));
    loop{
        let mut B=Array::zeros(u);
        for i in 0..q{
            B=xor(&B, &xor(&A.slice(s![i,..]).to_owned(),&s.slice(s![y-1,i,..]).to_owned()));
        }
        let mut m =0;
        if B[[o-1]] ==1{
            for i in 0..u{
                if i== o-1{
                    continue;
                }
                m = B[[i]] | m;
            }
            if m==0{
                break;
            }
        }
        A = Array::random((q, u), Uniform::new(0, 2));
    }
    let mut res_to_vec = vec![]; 
    for row in A.genrows(){
        res_to_vec.push(row.to_vec());
    }
    return res_to_vec;
}

pub fn GenerateSigma(A:&Vec<Array2<i32>>,v:usize,s:&Array3<i32>,q:usize,p: usize, lambda: usize)->Vec<Vec<i32>>{
    let mut res1=Vec::new();
    for j in 0..p{
        let mut res2=Vec::new();
        for i in 0..v{
            for k in 0..q{
                if A[i][[j,k]]==1{
                    for m in 0..lambda{
                        res2.push(s[[i,k,m]])
                    }
                }else{
                    for m in 0..lambda{
                    res2.push(0)
                    }
                }
            }
        } 
        res1.push(res2);
    }
    
    return res1
}

pub fn Decompose(D:&DecomposeParam)-> QuerySlice{
    let N1=D.N as f32;
    let i1=D.i as u32;
    let (q,u,v,o,y)=Coefficient(D.p, N1, i1);
    let A=GenerateMatrix(D.p, q, y, v);
    let s=Generatestring_rand(D.lambda, v, q);
    let A3=PRNG(&s,u,v,q,D.lambda);
    let cw=Generatecw(&A3, u, q, o, y);
    let Sigma=GenerateSigma(&A, v, &s, q, D.p, D.lambda);
    let mut k=Vec::new();
    for i in 0..D.p{
        let mut k1=Vec::new();
        k1.push(Sigma[i].clone());
        for j in 0..q{
            k1.push(cw[j].clone());
        }
        k.push(k1);
    }
    let res=QuerySlice{
        slice:k.clone(),
    };
    res
}

fn PRNG1(s:Vec<i32>,lambda: usize,u:usize)->Vec<i32>{
    let mut res=vec!(0;u);
    let n=u/lambda;
    for m in 1..n+1{
        for j in (m-1)*lambda..m*lambda{
            res[j]=s[j-(m-1)*lambda];
        }
    }
    return res;
}
fn judge(a: &Vec<i32>, begin: usize, l:usize)->bool{
    let mut res=0;
    for i in begin..l+begin{
        res=a[i]|res;
        if res!=0{
            return true;
        }
    }
    return false;
}

fn GenerateD(cw:&Vec<Vec<i32>>,q:usize,Sigma:&Vec<i32>,p: usize,v:usize,u:usize,lambda: usize)->Vec<Vec<i32>>{
    let mut D=Vec::new();
    for i in 0..v{
        let mut D1=vec!(0;u);
        let mut n=0;
        let mut k=i*q*lambda;
        while k<(i+1)*q*lambda {
            if judge(&Sigma,k,lambda){
                D1=xor_vec(&D1,&xor_vec(&PRNG1(Sigma[k..k+lambda].to_vec(),lambda,u),&cw[n]));
            }
            n=n+1;
            k+=lambda
        }
        D.push(D1);
    }
    return D;
}

fn GenerateRespondy(D:&Vec<Vec<i32>>,N:usize,x:&Vec<Vec<i32>>,u:usize)->Vec<i32>{
    let mut Y=vec!(0;x[0].len());
    for i in 0..N{
        let deta=i%u;
        let gama=i/u;
        if D[gama][deta]==1{
            Y=xor_vec(&Y, &x[i])
        }
    }
    return Y
}
pub fn Response(k:&Vec<Vec<i32>>,p:usize,q:usize,u:usize,v:usize,lambda: usize,N:usize,x:&Vec<Vec<i32>>)->Vec<i32>{
    let Sigma=k[0].clone();
    let mut cw=Vec::new();
    for i in 1..k.len(){
        cw.push(k[i].clone());
    }
    let D=GenerateD(&cw, q, &Sigma, p, v, u, lambda);
    let Y=GenerateRespondy(&D, N, x, u);
    return Y
}
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AggregateResult{
    #[serde(rename = "result")]
    pub res:Vec<i32>
}
pub fn aggregate(Y:&ResultSlice)->AggregateResult{
    let mut result1=vec!(0;Y.slice[0].len());
    for i in 0..Y.len{
        result1 =xor_vec(&result1,&Y.slice[i]);
    }
    let result=AggregateResult { res: result1 };
    return result;
}