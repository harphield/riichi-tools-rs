use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use riichi_tools_rs::riichi::hand::Hand;

pub fn hand_from_text_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("text hands");
    for hand_string in [
        "333699m17p13678s8m",
        "279m34688p139s15z9p",
        "24457m68p7s12467z7s",
        "59m16p123589s123z3s",
        "39m7p24567s11234z7p",
        "179m2577p2368s17z1s",
        "18m1266p569s3344z4s",
        "29m23479p1356s37z1p",
        "177m247p13468s233z",
        "3m4456677p2588s57z",
        "148m2459p133489s1z",
        "4567m1368p3399s5z5m",
        "223778m355p78s37z1m",
        "12267m5668s12336z",
        "6m1556p3588s5667z3s",
        "157m23345p23559s8m",
        "245m114p2688s245z7s",
        "568m113345p48s57z3m",
        "378999m126p229s3z3p",
        "14669m336p79s246z9m",
        "4567m3p25569s247z2m",
        "14458899m68p3s56z7p",
        "6m2245689p234s45z4m",
        "448m78p356778s55z5s",
        "9m5p12446669s1351z",
        "236m3345569p48s6z1p",
        "2234m235p13668s4z9m",
        "4469m1478p3467s34z",
        "11367m1266p9s156z7s",
        "146m189p17999s25z7p",
        "456m3344p144569s7m",
        "123678m1239p3s56z7s",
        "3799m23446p45s56z1p",
        "146689m66p44s267z6m",
        "56m19p23699s3355z3m",
        "166m13379p789s143z",
        "1249m58p36s144663z",
        "5m5p34568s355566z8s",
        "3447m2678p689s23z1s",
        "24m1299p34669s35z9s",
        "6m233778p1247s16z1s",
        "168m3369p3457s474z",
        "26699m245677p897s",
        "236778m478p24s27z3m",
        "566m159p1337s245z9s",
        "234m44689p1236s4z7p",
        "79m5689p44679s45z6s",
        "3456m223p1s23356z1s",
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::from_parameter(&hand_string),
            hand_string,
            |b, hand_string| {
                b.iter(|| {
                    let _hand = Hand::from_text(hand_string, false).unwrap();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, hand_from_text_benchmark);
criterion_main!(benches);
