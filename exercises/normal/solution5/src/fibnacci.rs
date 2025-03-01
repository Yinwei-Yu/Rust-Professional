pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut v=vec![0,1];
    while v[v.len()-1]+v[v.len()-2]<=threshold {
        v.push(v[v.len()-1]+v[v.len()-2]);
    }
    let mut sum=0;
    for num in v {
        if num%2!=0 {
            sum+=num;
        }
    }
    sum
}
