fn main() {
    println!("Hello, World!");
    let result = mendel(22.0, 24.0, 28.0);
    println!("{result}");
}

fn mendel(n: f32, m: f32, k: f32) -> f32 {
    // n:YY m:Yy k:yy
    // Prob(Y) = Prob(YY)*Prob(YY)*4/4 + Prob(Yy) * Prob(Yy)*3/4 + Prob(YY)*Prob(yy)*2
    // Prob(homozygous) = Prob(Y) * Prob (y) + Prob(Y)*Prob(Y) = Prob(YY)*Prob(YY) + Prob(YY)*Prob(Yy) +
    // Prob(homozygous) := Prob(YY) + Prob(Yy) := Prob(YYp) * Prob(YYp) +++ Prob(YYp) * Prob(yyp) +++ 1/2[Prob(Yy) * Prob(Yy)]
    // = n**2 / (n + m + k)**2 + 1/2[m**2/(n + m + k)**2] +++ n*k / (n + m + k)**2 +++ (.5m**2) / (n + m + k)**2
    // = n**2 + n*k + .5m**2 / (n + m + k)**2
    // Prob(Yy)
    let _total_possible = (n + k + m) * (n + k + m - 1.0) / 2.0;
    //Prob YY dominant = Prob (YY) * 1 = n / (n + k + m) or n*(n+k+m) / denom
    //Prob Yy dominant = Prob Yy * YY = n*m / (n+k+m)**2 + 3/4*m**2 / denom + 1/2*m*k / denom
    //Prob yy dominant = Prob YY *yy = n*k/denom + 1/2*m*k / denom + 0
    // If YY, all pairs will be dominant
    // if Yy x YY, dominant. If Yy x Yy, 3/4 dominant, if Yy x yy, 2/4 dominant
    // if yy x YY, dominant, if yy x Yy, 2/4, if yy x yy, 0/4
    // Prob(Yyp)*Prob(yyp) = Yy,Yy,yy,yy = 1/2[m*k]
    let homozygous_dominant = n * (n - 1.0) + n * m + n * k;
    let heterozygous = n * m + 0.75 * m * (m - 1.0) + 0.5 * m * k;
    let homozygous_recessive = n * k + 0.5 * m * k;
    let numerator = homozygous_dominant + heterozygous + homozygous_recessive;
    let denom = (n + m + k) * (n + m + k - 1.0);
    numerator / denom
}
