
use chain_demo::dpf::*;

fn main(){
    let p:usize=4; //发送给p个服务器
    let lambda = 8; //随机字符串长度为lamba
    let N:usize=8; //总数据量N
    let i:usize=7; //查询第i个数据
    let x=vec![vec![1,0,1,1,1,1,1,1],vec![1,1,0,0,1,1,1,1],vec![1,0,1,0,1,1,1,1],vec![0,0,1,1,1,1,1,1],vec![0,1,1,1,1,1,1,1],vec![0,1,1,0,1,1,1,1],vec![0,1,0,1,1,1,1,1],vec![0,1,0,0,1,1,1,1]];
    println!("查询索引i:{:?}\n",i);
    println!("数据:{:?}\n",x);
    let D=DecomposeParam{
        i:i,
        p:p,
        N:N,
        lambda:lambda
    };
    let (k)=Decompose(&D);
    println!("k{:?}\n",k.slice[1]);
    let mut Y=Vec::new();
    for i in 0..p{
        let (q,u,v)=Coefficient_server(p, N);
        Y.push(Response(&k.slice[i], p, q, u, v, lambda, N, &x));
    }
    let Res=ResultSlice{
        slice:Y,
        len:p,
    };
    let mut result=aggregate(&Res);
    println!("result:{:?}\n",result);
    println!("对应答案:{:?}\n",x[i-1]);
}