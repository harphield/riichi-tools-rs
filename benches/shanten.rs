use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use riichi_tools_rs::riichi::hand::Hand;

pub fn shanten_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shanten hands");
    for hand in [
        Hand::from_text("333699m17p13678s8m", false).unwrap(),
        Hand::from_text("279m34688p139s15z9p", false).unwrap(),
        Hand::from_text("24457m68p7s12467z7s", false).unwrap(),
        Hand::from_text("59m16p123589s123z3s", false).unwrap(),
        Hand::from_text("39m7p24567s11234z7p", false).unwrap(),
        Hand::from_text("179m2577p2368s17z1s", false).unwrap(),
        Hand::from_text("18m1266p569s3344z4s", false).unwrap(),
        Hand::from_text("29m23479p1356s37z1p", false).unwrap(),
        Hand::from_text("177m247p13468s233z", false).unwrap(),
        Hand::from_text("3m4456677p2588s57z", false).unwrap(),
        Hand::from_text("148m2459p133489s1z", false).unwrap(),
        Hand::from_text("4567m1368p3399s5z5m", false).unwrap(),
        Hand::from_text("223778m355p78s37z1m", false).unwrap(),
        Hand::from_text("12267m5668s12336z", false).unwrap(),
        Hand::from_text("6m1556p3588s5667z3s", false).unwrap(),
        Hand::from_text("157m23345p23559s8m", false).unwrap(),
        Hand::from_text("245m114p2688s245z7s", false).unwrap(),
        Hand::from_text("568m113345p48s57z3m", false).unwrap(),
        Hand::from_text("378999m126p229s3z3p", false).unwrap(),
        Hand::from_text("14669m336p79s246z9m", false).unwrap(),
        Hand::from_text("4567m3p25569s247z2m", false).unwrap(),
        Hand::from_text("14458899m68p3s56z7p", false).unwrap(),
        Hand::from_text("6m2245689p234s45z4m", false).unwrap(),
        Hand::from_text("448m78p356778s55z5s", false).unwrap(),
        Hand::from_text("9m5p12446669s1351z", false).unwrap(),
        Hand::from_text("236m3345569p48s6z1p", false).unwrap(),
        Hand::from_text("2234m235p13668s4z9m", false).unwrap(),
        Hand::from_text("4469m1478p3467s34z", false).unwrap(),
        Hand::from_text("11367m1266p9s156z7s", false).unwrap(),
        Hand::from_text("146m189p17999s25z7p", false).unwrap(),
        Hand::from_text("456m3344p144569s7m", false).unwrap(),
        Hand::from_text("123678m1239p3s56z7s", false).unwrap(),
        Hand::from_text("3799m23446p45s56z1p", false).unwrap(),
        Hand::from_text("146689m66p44s267z6m", false).unwrap(),
        Hand::from_text("56m19p23699s3355z3m", false).unwrap(),
        Hand::from_text("166m13379p789s143z", false).unwrap(),
        Hand::from_text("1249m58p36s144663z", false).unwrap(),
        Hand::from_text("5m5p34568s355566z8s", false).unwrap(),
        Hand::from_text("3447m2678p689s23z1s", false).unwrap(),
        Hand::from_text("24m1299p34669s35z9s", false).unwrap(),
        Hand::from_text("6m233778p1247s16z1s", false).unwrap(),
        Hand::from_text("168m3369p3457s474z", false).unwrap(),
        Hand::from_text("26699m245677p897s", false).unwrap(),
        Hand::from_text("236778m478p24s27z3m", false).unwrap(),
        Hand::from_text("566m159p1337s245z9s", false).unwrap(),
        Hand::from_text("234m44689p1236s4z7p", false).unwrap(),
        Hand::from_text("79m5689p44679s45z6s", false).unwrap(),
        Hand::from_text("3456m223p1s23356z1s", false).unwrap(),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::from_parameter(hand.to_string()),
            hand,
            |b, hand| {
                b.iter(|| {
                    hand.get_shanten();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, shanten_benchmark);
criterion_main!(benches);
