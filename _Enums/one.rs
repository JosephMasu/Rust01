

#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6,
}

fn main(){
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IPAddrKind){
   println!( "{:?}", ip_kind) 
}