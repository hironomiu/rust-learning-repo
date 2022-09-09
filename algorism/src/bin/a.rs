fn main() {
    let input = vec![vec![1, 0, 1], vec![2, 1, 2], vec![1, 0, 1]];

    println!("整数 a1,a2,a3,b1,b2,b3、マス(i,j) -> ai + bi");
    println!("input: {:?}", input);

    let result = vec![
        vec![
            input[0][0] - input[1][0],
            input[0][1] - input[1][1],
            input[0][2] - input[1][2],
        ],
        vec![
            input[1][0] - input[2][0],
            input[1][1] - input[2][1],
            input[1][2] - input[2][2],
        ],
    ];
    if (result[0][0] == result[0][1] && result[0][1] == result[0][2])
        && (result[1][0] == result[1][1] && result[1][1] == result[1][2])
    {
        println!("YES");
    } else {
        println!("NO");
    }
    println!("{:?}", result);
}
