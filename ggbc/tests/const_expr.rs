// generated (ggbc/src/bin/const_expr_gen.sh)
use ggbc::{byteorder::NativeEndian, ir::Ir, parser::parse};
use ggbc_vm::{Machine, Opts};

fn test_const_expr(input: &str) {
    let ast = parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    let result = Machine::new(&ir, Opts::default()).run().static_[0];
    assert_eq!(0xff, result);
}
#[test]
fn const_expr_1() {
    test_const_expr(r#"
const c12:u8=242
const c16:u8=211
const c30:u8=0
static RESULT:u8
(= RESULT (- (+ (+ (- c12 c16) 32) 192) c30))
"#);
}
#[test]
fn const_expr_2() {
    test_const_expr(r#"
const c12:u8=235
const c19:u8=99
const c29:u8=221
const c33:u8=121
const c44:u8=214
const c54:u8=239
const c64:u8=5
const c90:u8=254
const c104:u8=1
const c110:u8=9
const c117:u8=51
const c135:u8=187
const c144:u8=219
const c150:u8=0
const c156:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- c12 (+ c19 (+ (- c29 c33) (- (- c44 (- (- c54 (+ (+ c64 (- 206 189)) (- (+ (- c90 (- (- 254 c104) c110)) c117) 38))) 167)) c135)))) c144) c150) c156))
"#);
}
#[test]
fn const_expr_3() {
    test_const_expr(r#"
const c6:u8=213
static RESULT:u8
(= RESULT (+ 42 c6))
"#);
}
#[test]
fn const_expr_4() {
    test_const_expr(r#"
const c6:u8=36
const c12:u8=235
const c16:u8=16
static RESULT:u8
(= RESULT (- (+ c6 (- c12 c16)) 0))
"#);
}
#[test]
fn const_expr_5() {
    test_const_expr(r#"
const c19:u8=235
const c28:u8=120
const c33:u8=90
static RESULT:u8
(= RESULT (+ 127 (- (- (+ (- c19 115) c28) c33) 22)))
"#);
}
#[test]
fn const_expr_6() {
    test_const_expr(r#"
const c3:u8=36
const c12:u8=10
const c16:u8=33
const c21:u8=176
static RESULT:u8
(= RESULT (+ c3 (+ (+ c12 c16) c21)))
"#);
}
#[test]
fn const_expr_7() {
    test_const_expr(r#"
const c21:u8=250
const c41:u8=239
const c45:u8=201
const c67:u8=162
const c78:u8=3
const c112:u8=50
const c121:u8=32
const c126:u8=65
const c137:u8=20
const c155:u8=93
const c168:u8=131
const c175:u8=56
const c197:u8=68
const c202:u8=14
const c217:u8=0
const c222:u8=0
static RESULT:u8
(= RESULT (- (+ (+ (- (+ (- (- c21 (- (- (- 244 (- c41 c45)) (+ 13 (+ (- 167 c67) (+ (+ c78 (- 124 124)) (+ (+ 3 (- (+ 50 c112) (+ c121 c126))) (- c137 6)))))) 70)) c155) (- 131 c168)) c175) (+ 30 0)) (+ (- c197 c202) 165)) (+ c217 c222)))
"#);
}
#[test]
fn const_expr_8() {
    test_const_expr(r#"
const c3:u8=63
static RESULT:u8
(= RESULT (+ c3 192))
"#);
}
#[test]
fn const_expr_9() {
    test_const_expr(r#"
const c12:u8=42
const c52:u8=15
const c56:u8=16
const c61:u8=125
const c72:u8=96
static RESULT:u8
(= RESULT (- (+ (- (+ c12 (- 237 (- 228 (- 176 (+ 28 (- (+ (+ c52 c56) c61) (+ 32 c72))))))) 86) (+ 34 136)) 0))
"#);
}
#[test]
fn const_expr_10() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 85 170))
"#);
}
#[test]
fn const_expr_11() {
    test_const_expr(r#"
const c12:u8=94
const c31:u8=41
const c35:u8=14
const c46:u8=33
const c70:u8=22
const c88:u8=237
const c107:u8=230
const c112:u8=44
const c132:u8=7
const c147:u8=97
const c179:u8=175
const c204:u8=22
const c209:u8=92
const c225:u8=72
const c239:u8=254
const c253:u8=20
const c265:u8=157
const c292:u8=224
const c297:u8=222
const c304:u8=5
const c310:u8=21
const c329:u8=2
const c340:u8=0
const c352:u8=92
const c378:u8=232
const c392:u8=30
const c408:u8=8
const c424:u8=36
const c429:u8=36
const c441:u8=6
const c454:u8=0
const c462:u8=171
const c485:u8=86
const c511:u8=254
const c516:u8=0
const c542:u8=18
const c551:u8=17
const c561:u8=54
const c576:u8=202
const c588:u8=201
const c593:u8=37
const c608:u8=3
const c619:u8=1
const c627:u8=0
const c632:u8=2
const c639:u8=0
const c650:u8=49
const c664:u8=252
const c691:u8=122
const c716:u8=2
const c731:u8=36
const c736:u8=180
const c742:u8=71
const c754:u8=7
const c765:u8=183
const c801:u8=24
const c819:u8=11
const c831:u8=0
const c845:u8=42
const c855:u8=14
const c860:u8=56
const c872:u8=0
static RESULT:u8
(= RESULT (- (+ (+ (- c12 (+ (+ (+ (- (- c31 c35) (+ (- c46 (+ 28 (- 118 118))) c70)) (+ 1 (- 240 c88))) (- (- (- (- c107 c112) (+ (+ (+ 0 3) c132) 33)) 29) c147)) (+ (- 129 (+ 16 (+ 16 (- c179 142)))) (+ (+ (- (+ c204 c209) (- 149 (- c225 (+ (- (- c239 2) (+ (+ c253 (- 218 c265)) 162)) (+ (+ (+ 0 (- c292 c297)) c304) c310))))) (- 15 (+ c329 13))) c340)))) (- c352 65)) (- (+ 224 (- (- c378 (- (+ (+ c392 181) (+ (- c408 (+ (+ 1 (- c424 c429)) (+ 1 c441))) (+ 0 c454))) c462)) (- (+ (+ 241 (- c485 (- (+ (- (- (- (- (- c511 c516) 0) 20) (- (- (- 113 c542) (+ c551 18)) c561)) 178) (- c576 2)) (- c588 c593)))) (+ (- c608 (+ (+ c619 (+ c627 c632)) c639)) 0)) c650))) (- (+ c664 0) (+ 123 (+ (+ (+ (- c691 (- (- (- (- 253 15) c716) (- (- (+ c731 c736) c742) (+ (+ c754 (- (- c765 144) 31)) 15))) (- 55 55))) 6) c801) (+ (- (+ (+ c819 60) (+ c831 0)) (+ 6 c845)) (+ c855 c860))))))) c872))
"#);
}
#[test]
fn const_expr_12() {
    test_const_expr(r#"
const c3:u8=36
const c12:u8=249
const c27:u8=51
const c37:u8=134
const c52:u8=30
const c69:u8=183
const c73:u8=27
const c89:u8=202
const c93:u8=17
static RESULT:u8
(= RESULT (+ c3 (- (- c12 (+ 0 (- (+ c27 (+ 22 c37)) (+ (+ (- c52 5) (- 181 (- c69 c73))) (+ (- (- c89 c93) 34) 0))))) 24)))
"#);
}
#[test]
fn const_expr_13() {
    test_const_expr(r#"
const c9:u8=51
const c15:u8=68
const c19:u8=136
const c31:u8=86
const c40:u8=0
const c47:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 (+ c15 c19)) (- 86 c31)) (+ c40 (+ c47 0))))
"#);
}
#[test]
fn const_expr_14() {
    test_const_expr(r#"
const c21:u8=127
const c25:u8=128
const c48:u8=194
const c52:u8=8
const c62:u8=112
const c75:u8=0
const c83:u8=142
const c93:u8=28
const c118:u8=37
const c136:u8=8
const c141:u8=17
const c170:u8=121
const c175:u8=121
const c181:u8=234
const c187:u8=44
const c201:u8=0
const c209:u8=0
static RESULT:u8
(= RESULT (- (+ (- (- (- (+ (+ c21 c25) (- (- (+ 41 (- (- c48 c52) 22)) c62) 93)) 0) c75) (- c83 (- (+ c93 143) (+ (- 192 (- (+ c118 189) 38)) (+ c136 c141))))) (- (- 185 (+ (- (+ c170 c175) c181) c187)) (+ 133 c201))) c209))
"#);
}
#[test]
fn const_expr_15() {
    test_const_expr(r#"
const c12:u8=63
const c74:u8=48
const c78:u8=48
const c94:u8=58
const c104:u8=8
const c127:u8=169
const c137:u8=48
const c150:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ c12 (- (- (- (+ (- (+ 32 (- 146 (- (+ 54 110) 146))) (+ 15 (+ c74 c78))) (+ (+ (- c94 (+ (+ c104 (+ 0 0)) 41)) 19) c127)) 2) c137) 4)) 0) c150) 0))
"#);
}
#[test]
fn const_expr_16() {
    test_const_expr(r#"
const c9:u8=36
const c22:u8=0
const c30:u8=13
const c53:u8=46
static RESULT:u8
(= RESULT (+ (- (+ c9 (- (+ 173 c22) (+ c30 14))) (+ (- 25 16) c53)) 128))
"#);
}
#[test]
fn const_expr_17() {
    test_const_expr(r#"
const c6:u8=63
const c15:u8=253
const c19:u8=6
const c34:u8=0
static RESULT:u8
(= RESULT (+ (+ c6 (- (- c15 c19) 55)) (+ 0 c34)))
"#);
}
#[test]
fn const_expr_18() {
    test_const_expr(r#"
const c6:u8=192
static RESULT:u8
(= RESULT (+ 63 c6))
"#);
}
#[test]
fn const_expr_19() {
    test_const_expr(r#"
const c24:u8=102
const c37:u8=250
const c58:u8=131
const c85:u8=174
const c109:u8=45
const c124:u8=238
const c158:u8=80
const c170:u8=129
const c199:u8=34
const c219:u8=134
const c228:u8=229
const c244:u8=0
const c252:u8=181
const c277:u8=48
const c282:u8=0
const c316:u8=6
const c341:u8=12
const c362:u8=146
const c367:u8=57
const c412:u8=46
const c427:u8=5
const c448:u8=107
const c470:u8=30
const c486:u8=17
static RESULT:u8
(= RESULT (+ (- (- (- (+ (- (+ (- c24 (- (- (- c37 (- (- (+ 39 198) c58) 105)) (+ (- 241 (+ 58 c85)) 28)) 148)) (- 200 c109)) 135) (- c124 61)) (+ (- (+ 15 (- 118 39)) c158) (+ (- c170 (- (+ (+ (+ (+ (- (- 74 c199) (+ 8 (- (+ 22 c219) (- c228 105)))) (+ c244 (- c252 180))) 4) 30) (+ (+ c277 c282) (+ (+ 24 (- (+ 22 (+ (- 139 c316) (+ 0 0))) (+ (+ (- c341 9) 19) (+ 44 (- c362 c367))))) (+ 14 (+ (+ 2 12) 44))))) (- (- (+ c412 185) (+ 4 c427)) 162))) (- 140 c448)))) 8) (+ 8 (+ 5 c470))) (+ (+ 8 c486) 103)))
"#);
}
#[test]
fn const_expr_20() {
    test_const_expr(r#"
const c6:u8=63
const c36:u8=224
const c42:u8=31
const c80:u8=42
const c90:u8=64
const c121:u8=25
const c127:u8=62
const c141:u8=180
const c158:u8=88
const c167:u8=74
const c183:u8=19
const c224:u8=55
const c233:u8=27
const c242:u8=159
const c250:u8=0
const c281:u8=4
const c293:u8=241
const c321:u8=92
const c339:u8=18
const c357:u8=252
const c385:u8=227
const c400:u8=3
const c408:u8=2
const c413:u8=10
const c423:u8=136
const c428:u8=73
const c442:u8=10
const c453:u8=2
const c458:u8=8
const c494:u8=24
const c503:u8=17
const c527:u8=66
const c551:u8=82
const c558:u8=90
const c581:u8=38
const c589:u8=64
const c600:u8=46
const c626:u8=15
const c635:u8=47
const c675:u8=17
const c707:u8=225
const c712:u8=19
const c728:u8=112
const c737:u8=42
const c742:u8=85
const c768:u8=167
const c773:u8=3
const c783:u8=155
const c797:u8=70
const c813:u8=73
const c823:u8=5
const c834:u8=0
const c839:u8=1
const c845:u8=10
const c871:u8=0
const c876:u8=0
const c891:u8=97
static RESULT:u8
(= RESULT (- (+ c6 (+ (- 252 (- (- 230 (- 235 c36)) c42)) 128)) (- (- 227 (- 139 (- (+ (- c80 (- (- c90 (+ (- (+ (- (- 178 (+ (+ 5 c121) c127)) 67) (- c141 (+ 42 42))) c158) (- c167 74))) (+ 9 c183))) (+ 33 135)) (- (+ (- (- (+ (+ (- c224 51) c233) (+ c242 (+ c250 0))) (+ (+ (- 169 165) (+ c281 (- 246 c293))) 65)) (+ (- (+ (+ 92 c321) 0) (+ (+ (+ c339 (+ (- 199 (- c357 90)) 0)) 111) (- (- (- c385 28) (+ (+ c400 (+ c408 c413)) (- c423 c428))) (+ (+ c442 (+ (+ c453 c458) (- 214 174))) (- 138 (+ (- 49 c494) (+ c503 35))))))) (- (+ (- c527 (+ (- (+ 13 (- 160 c551)) c558) 11)) (+ (- (- (+ c581 (+ c589 130)) c600) 159) (- (+ (+ (- 30 c626) (+ c635 0)) 63) (+ (- 210 188) (+ (- 10 5) c675))))) (+ (- (+ (+ (- 210 (- c707 c712)) (- (+ 27 c728) (+ c737 c742))) (- (+ (+ 41 0) (- c768 c773)) (+ c783 0))) 49) c797)))) (- 169 c813)) (+ c823 (+ (+ c834 c839) c845)))))) (- (+ (+ 83 (+ c871 c876)) 166) (- c891 38)))))
"#);
}
#[test]
fn const_expr_21() {
    test_const_expr(r#"
const c9:u8=127
const c12:u8=128
const c32:u8=251
const c44:u8=229
const c64:u8=248
const c83:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 c12) (+ (+ (- (+ (- c32 162) (- c44 229)) 89) (- (- c64 119) 129)) 0)) c83))
"#);
}
#[test]
fn const_expr_22() {
    test_const_expr(r#"
const c12:u8=9
const c22:u8=241
const c35:u8=157
const c78:u8=0
const c84:u8=0
const c106:u8=245
const c111:u8=0
const c133:u8=132
const c163:u8=6
const c184:u8=147
const c191:u8=0
const c211:u8=192
const c240:u8=0
const c276:u8=7
const c287:u8=77
const c307:u8=36
const c325:u8=28
const c347:u8=75
const c364:u8=72
const c373:u8=207
const c378:u8=35
const c391:u8=208
const c413:u8=143
const c419:u8=90
const c432:u8=60
const c452:u8=126
const c466:u8=94
const c491:u8=10
const c503:u8=6
const c508:u8=18
const c540:u8=8
const c553:u8=56
const c558:u8=45
const c574:u8=223
const c579:u8=92
const c585:u8=2
const c612:u8=49
const c630:u8=141
const c656:u8=41
const c666:u8=23
const c694:u8=226
const c715:u8=75
const c724:u8=184
const c749:u8=254
const c768:u8=36
const c773:u8=37
const c805:u8=13
const c829:u8=50
const c834:u8=101
const c841:u8=20
const c885:u8=83
const c893:u8=174
const c898:u8=152
const c908:u8=7
const c949:u8=184
const c962:u8=165
const c978:u8=140
const c990:u8=254
const c1006:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ c12 (+ (- c22 (- (+ 78 c35) (- (- (+ (- (- 183 (+ 9 (+ (+ 19 (+ 0 c78)) c84))) 89) 132) (- (- c106 c111) (+ (- (- 187 (- c133 52)) 0) (+ (+ 0 (- (+ (+ c163 18) (- 214 91)) c184)) c191)))) 39))) 27)) c211) (+ (- 72 72) 0)) (+ (+ c240 (- 141 141)) (+ (- (- (+ (+ (+ c276 8) (- c287 (+ (- (- (- (+ c307 (- (- 252 6) c325)) (+ (- (+ (+ 24 c347) 0) 98) 3)) c364) (- c373 c378)) (- (- c391 (+ (- (+ 31 126) c413) c419)) (+ 20 c432))))) (- 158 (- c452 (- (+ 15 c466) (- (+ (- (- (- 141 c491) (- (+ c503 c508) 17)) (+ (+ (- (+ 6 24) (+ c540 17)) (- c553 c558)) (- (- (- c574 c579) c585) 46))) 126) (- 234 (+ c612 (- 214 (+ 23 c630))))))))) (- (- (- (+ c656 206) c666) (+ (+ (- (- 108 (- (- c694 (- 119 (+ (+ 37 c715) (- c724 184)))) (- (+ 36 (- c749 36)) (- (+ (+ c768 c773) 74) (- 142 112))))) (+ (+ c805 (- 168 (- (+ 37 (+ c829 c834)) c841))) 13)) 2) (- (+ (- (- (+ 82 82) (- (- c885 (- c893 c898)) (+ c908 (+ 3 (+ 1 10))))) 117) (+ 6 18)) (- c949 166)))) c962)) (- (+ 69 c978) (- (- c990 144) 44))) c1006))))
"#);
}
#[test]
fn const_expr_23() {
    test_const_expr(r#"
const c6:u8=173
const c26:u8=211
const c30:u8=9
const c38:u8=213
static RESULT:u8
(= RESULT (+ (- c6 (- 164 (- 235 (- c26 c30)))) c38))
"#);
}
#[test]
fn const_expr_24() {
    test_const_expr(r#"
const c24:u8=0
const c33:u8=0
static RESULT:u8
(= RESULT (- (+ 63 (- 204 12)) (+ c24 (+ 0 c33))))
"#);
}
#[test]
fn const_expr_25() {
    test_const_expr(r#"
const c6:u8=10
const c9:u8=53
const c22:u8=12
const c26:u8=0
const c38:u8=240
static RESULT:u8
(= RESULT (+ (+ c6 c9) (+ (+ (+ c22 c26) 36) (- c38 96))))
"#);
}
#[test]
fn const_expr_26() {
    test_const_expr(r#"
const c17:u8=9
const c58:u8=15
const c78:u8=1
const c82:u8=7
const c93:u8=58
const c112:u8=0
const c120:u8=0
const c134:u8=178
const c162:u8=176
const c168:u8=46
const c199:u8=242
const c238:u8=40
const c260:u8=84
const c274:u8=86
const c286:u8=0
const c297:u8=182
const c305:u8=215
const c316:u8=139
const c337:u8=40
const c343:u8=11
const c351:u8=0
static RESULT:u8
(= RESULT (- (+ (+ (+ (+ 3 c17) (+ (+ (- (- 226 (+ (+ 0 (+ (- (+ (+ c58 77) (- 43 (+ (+ c78 c82) (- 93 c93)))) 92) 0)) (+ c112 (+ c120 (- 96 (- c134 (+ (- 62 49) (+ (- 199 c162) c168)))))))) 224) (- 146 (+ (- c199 220) (+ 16 96)))) 59)) 170) (- (+ c238 163) (- (- (- (+ c260 (- (+ 85 c274) (+ (+ c286 (- (- c297 (- c305 172)) c316)) 1))) (+ 0 0)) c337) c343))) c351))
"#);
}
#[test]
fn const_expr_27() {
    test_const_expr(r#"
const c12:u8=36
const c32:u8=178
const c42:u8=148
const c57:u8=43
const c61:u8=172
const c73:u8=57
const c77:u8=58
const c90:u8=97
const c94:u8=51
const c113:u8=253
const c142:u8=11
const c163:u8=60
const c179:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ c12 (+ 109 (+ (+ (- c32 (+ 24 c42)) (- (- (+ c57 c61) 88) (+ c73 c77))) (+ (- c90 c94) 46)))) (- 253 c113)) (- 134 134)) (+ (+ (- c142 11) (- 75 (+ 15 c163))) (+ 0 (+ c179 0)))))
"#);
}
#[test]
fn const_expr_28() {
    test_const_expr(r#"
const c12:u8=219
const c20:u8=0
static RESULT:u8
(= RESULT (- (+ (+ 36 c12) 0) c20))
"#);
}
#[test]
fn const_expr_29() {
    test_const_expr(r#"
const c18:u8=0
const c47:u8=0
const c56:u8=0
const c81:u8=13
const c85:u8=78
const c105:u8=250
const c110:u8=5
const c116:u8=245
const c122:u8=0
const c133:u8=60
const c139:u8=68
const c157:u8=207
const c175:u8=63
const c192:u8=20
const c204:u8=8
const c209:u8=33
const c219:u8=158
const c224:u8=158
const c247:u8=32
const c261:u8=15
const c271:u8=13
const c283:u8=108
const c294:u8=72
const c317:u8=46
const c325:u8=252
const c330:u8=67
const c337:u8=9
const c358:u8=66
const c409:u8=212
const c423:u8=138
const c429:u8=154
const c470:u8=12
const c487:u8=46
const c494:u8=29
static RESULT:u8
(= RESULT (+ (+ (- (+ 77 (+ c18 (+ (+ (+ 0 0) (+ (+ 0 (+ c47 (+ 0 c56))) (- 68 68))) (- (+ c81 c85) (+ 91 (+ (- (- c105 c110) c116) c122)))))) c133) c139) (+ (- (+ 34 c157) (- (- (- (+ c175 (+ (+ (+ (- c192 14) (+ c204 c209)) (- c219 c224)) (- (+ (- (- 252 c247) (+ 4 (- c261 (+ 2 c271)))) (- c283 108)) c294))) 7) (- (- (- (+ c317 (- c325 c330)) c337) (+ (- (+ (+ (- c358 (+ 10 (- 168 (- (+ 42 169) 97)))) (- (+ 50 (- c409 (- (+ 27 c423) c429))) (- 253 (- 233 (- 254 31))))) 11) c470) (+ 2 18))) c487)) c494)) (+ 0 0))))
"#);
}
#[test]
fn const_expr_30() {
    test_const_expr(r#"
const c32:u8=54
const c42:u8=6
const c53:u8=179
const c65:u8=20
const c86:u8=30
const c110:u8=6
const c120:u8=217
const c125:u8=75
const c133:u8=132
const c143:u8=216
const c164:u8=5
static RESULT:u8
(= RESULT (+ (- (+ (- 118 52) (- (- (+ (- c32 (+ (+ c42 12) (- c53 (+ (+ 5 c65) (+ (- (- 196 (+ c86 61)) (- (- 231 (+ 0 c110)) (- c120 c125))) c133))))) c143) (+ 1 2)) 183)) c164) 128))
"#);
}
#[test]
fn const_expr_31() {
    test_const_expr(r#"
const c18:u8=6
const c28:u8=5
const c51:u8=164
const c82:u8=0
const c95:u8=98
const c99:u8=76
const c107:u8=113
const c112:u8=0
const c119:u8=135
static RESULT:u8
(= RESULT (- (- (+ (+ 42 (+ c18 (- (+ c28 36) 4))) (+ (- 249 c51) 85)) (- 228 228)) (+ 0 (+ c82 (- (+ (- c95 c99) (+ c107 c112)) c119)))))
"#);
}
#[test]
fn const_expr_32() {
    test_const_expr(r#"
const c19:u8=0
static RESULT:u8
(= RESULT (- (+ 63 192) (+ 0 c19)))
"#);
}
#[test]
fn const_expr_33() {
    test_const_expr(r#"
const c12:u8=127
const c28:u8=41
const c32:u8=168
const c45:u8=56
const c65:u8=0
const c77:u8=0
const c81:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ c12 (- (- (- (+ c28 c32) 14) 11) c45)) (+ (+ 0 0) (+ c65 0))) (+ c77 c81)) 0))
"#);
}
#[test]
fn const_expr_34() {
    test_const_expr(r#"
const c13:u8=251
const c17:u8=15
const c29:u8=47
static RESULT:u8
(= RESULT (+ 127 (- (- c13 c17) (- 155 c29))))
"#);
}
#[test]
fn const_expr_35() {
    test_const_expr(r#"
const c35:u8=25
const c47:u8=7
const c62:u8=0
const c73:u8=103
const c95:u8=21
const c102:u8=0
static RESULT:u8
(= RESULT (- (+ 36 (- (+ (+ 61 0) 184) (+ (- c35 (+ 6 (+ c47 (+ (+ 1 (+ c62 (- 105 c73))) (- 101 97))))) c95))) c102))
"#);
}
#[test]
fn const_expr_36() {
    test_const_expr(r#"
const c12:u8=235
const c22:u8=245
const c26:u8=0
const c31:u8=61
const c37:u8=204
const c42:u8=0
const c53:u8=72
const c66:u8=80
const c71:u8=34
const c77:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- c12 (- (+ c22 c26) c31)) c37) c42) (+ (- c53 (- (+ 26 c66) c71)) c77)))
"#);
}
#[test]
fn const_expr_37() {
    test_const_expr(r#"
const c44:u8=87
const c57:u8=49
const c77:u8=52
const c81:u8=158
const c86:u8=16
const c91:u8=160
const c115:u8=192
const c146:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (+ (- (- 189 134) (- 72 29)) (- c44 (- (- (+ c57 (- 234 (- (- (+ c77 c81) c86) c91))) 89) (+ 124 0)))) c115) 0) (+ (- 226 (+ 45 181)) c146)) 0))
"#);
}
#[test]
fn const_expr_38() {
    test_const_expr(r#"
const c18:u8=2
const c26:u8=28
const c52:u8=7
const c60:u8=201
const c65:u8=7
const c86:u8=216
const c93:u8=27
const c97:u8=109
const c140:u8=156
const c148:u8=20
const c165:u8=33
const c177:u8=45
const c182:u8=40
const c192:u8=35
const c204:u8=249
const c246:u8=26
const c266:u8=17
const c283:u8=215
const c288:u8=2
const c329:u8=158
const c343:u8=34
const c363:u8=56
static RESULT:u8
(= RESULT (- (- (+ (+ (+ (+ c18 12) c26) (+ 42 (- (- (- (+ (+ c52 43) c60) c65) (- (- 157 (- (- c86 (+ c93 c97)) 15)) (- (- (+ 26 (- (- 230 51) (+ (- c140 (+ c148 126)) (+ (- c165 28) (- c177 c182))))) c192) (- (- c204 11) (- 241 65))))) 197))) (- (+ (+ 6 c246) 132) (- (+ (+ c266 18) 214) (- c283 c288)))) 0) (- (- (+ 49 147) (+ 1 (- 168 c329))) (- (+ c343 (+ 51 156)) (+ c363 0)))))
"#);
}
#[test]
fn const_expr_39() {
    test_const_expr(r#"
const c9:u8=204
const c19:u8=0
static RESULT:u8
(= RESULT (- (+ 51 c9) (+ (+ c19 0) 0)))
"#);
}
#[test]
fn const_expr_40() {
    test_const_expr(r#"
const c17:u8=16
static RESULT:u8
(= RESULT (- (+ 127 (- 144 c17)) 0))
"#);
}
#[test]
fn const_expr_41() {
    test_const_expr(r#"
const c37:u8=13
const c41:u8=67
const c52:u8=238
const c56:u8=185
const c73:u8=154
const c105:u8=28
const c125:u8=164
const c135:u8=3
const c154:u8=2
const c169:u8=0
const c174:u8=0
const c185:u8=1
const c202:u8=1
const c208:u8=0
const c223:u8=192
static RESULT:u8
(= RESULT (+ (- (- (+ (- (- (+ (+ (- 253 (+ (+ c37 c41) (+ (- c52 c56) (+ 21 (- (+ c73 0) 67))))) 51) (+ (- 207 (- c105 (+ (+ (- (+ (- c125 162) c135) (- 245 240)) c154) 10))) (+ c169 c174))) (+ c185 (- 64 64))) c202) c208) 87) 102) c223))
"#);
}
#[test]
fn const_expr_42() {
    test_const_expr(r#"
const c18:u8=135
const c52:u8=16
const c56:u8=13
const c73:u8=43
const c77:u8=43
const c82:u8=0
const c87:u8=2
const c95:u8=1
const c119:u8=227
const c124:u8=206
const c131:u8=171
const c150:u8=4
const c164:u8=8
const c180:u8=198
const c195:u8=0
const c215:u8=180
const c230:u8=23
const c235:u8=18
const c244:u8=0
const c250:u8=0
const c284:u8=23
const c312:u8=36
const c333:u8=129
const c351:u8=147
const c360:u8=97
const c372:u8=0
const c377:u8=0
const c383:u8=0
const c413:u8=40
const c430:u8=96
const c435:u8=97
const c463:u8=5
const c471:u8=30
const c476:u8=3
const c492:u8=166
const c497:u8=26
const c508:u8=66
const c524:u8=7
const c535:u8=245
const c543:u8=27
const c558:u8=161
const c580:u8=194
const c605:u8=206
const c620:u8=126
const c630:u8=17
const c647:u8=225
const c664:u8=66
const c687:u8=21
static RESULT:u8
(= RESULT (+ (+ 28 (+ 57 (- c18 (- (+ (- (- (- 225 (+ 3 (+ (- c52 c56) (+ (+ (+ (- c73 c77) c82) c87) (+ c95 (- (- (+ 99 100) (- c119 c124)) c131)))))) 112) (+ c150 (+ (- 18 c164) (+ (- 198 c180) (+ (+ (+ c195 (- (+ 3 (- 187 c215)) (+ 5 (- c230 c235)))) c244) c250))))) (- (+ (- (+ 78 0) (- (+ c284 115) (+ (+ 2 12) (+ 35 c312)))) (+ (+ 25 (+ c333 0)) (+ (- (- c351 50) c360) (+ (+ c372 c377) c383)))) 96)) 31)))) (- (- (+ c413 (+ (- (- (+ c430 c435) (+ 91 0)) (+ 15 (+ (+ c463 (- c471 c476)) 32))) (- c492 c497))) (- c508 61)) (+ (+ c524 (- (- c535 (+ c543 (- 243 (- c558 (- 121 (+ (- 207 c580) (- (+ 114 (+ (- (- c605 10) (+ 42 c620)) (+ c630 69))) (+ (- c647 (+ (+ 1 12) c664)) 0)))))))) 110)) c687))))
"#);
}
#[test]
fn const_expr_43() {
    test_const_expr(r#"
const c43:u8=61
const c66:u8=47
const c86:u8=90
const c98:u8=203
const c144:u8=223
const c200:u8=71
const c223:u8=6
const c233:u8=61
const c241:u8=45
static RESULT:u8
(= RESULT (+ 85 (- 184 (- 66 (+ (- (- (+ 66 (- (- (+ c43 184) 50) (- 108 (+ c66 0)))) 26) (+ (- c86 (+ 1 (- c98 194))) (+ (- (+ 11 (- 138 (+ (- 133 (- (- c144 (- (+ 17 89) (- 210 201))) (+ (- (- 205 44) (- 221 c200)) (+ (+ 0 3) (+ 3 c223))))) c233))) c241) 68))) 39)))))
"#);
}
#[test]
fn const_expr_44() {
    test_const_expr(r#"
const c9:u8=42
const c18:u8=232
const c22:u8=190
const c37:u8=11
const c41:u8=48
static RESULT:u8
(= RESULT (- (- (+ c9 (+ (- c18 c22) (- 230 (+ c37 c41)))) 0) (+ 0 0)))
"#);
}
#[test]
fn const_expr_45() {
    test_const_expr(r#"
const c3:u8=51
static RESULT:u8
(= RESULT (+ c3 204))
"#);
}
#[test]
fn const_expr_46() {
    test_const_expr(r#"
const c6:u8=244
const c31:u8=0
const c48:u8=125
const c59:u8=14
const c86:u8=248
const c100:u8=63
const c120:u8=0
const c128:u8=0
const c137:u8=98
const c144:u8=3
const c194:u8=6
const c200:u8=80
const c206:u8=163
const c218:u8=1
const c226:u8=4
const c236:u8=203
const c267:u8=0
const c281:u8=192
const c289:u8=230
const c299:u8=55
const c305:u8=109
const c315:u8=0
const c323:u8=80
const c328:u8=78
const c336:u8=4
const c342:u8=36
const c366:u8=36
const c374:u8=166
const c379:u8=94
const c406:u8=70
static RESULT:u8
(= RESULT (+ (- c6 (- (+ (+ (- (- 252 (+ c31 (+ (+ (- 126 c48) (- (+ c59 (+ (- (+ 20 120) (- (+ c86 0) (- 185 c100))) (+ 70 (+ (+ c120 0) c128)))) c137)) c144))) (- 230 (+ 7 31))) 170) (+ (+ (- (- (- 249 c194) c200) c206) (- (+ c218 3) c226)) (- c236 (+ (+ (+ 1 7) (+ (+ (+ (+ c267 (- (+ (- c281 (- c289 92)) c299) c305)) (+ c315 (- c323 c328))) c336) c342)) 153)))) (- (+ (+ c366 (- c374 c379)) (- 224 224)) (- 154 c406)))) 213))
"#);
}
#[test]
fn const_expr_47() {
    test_const_expr(r#"
const c12:u8=51
const c16:u8=204
const c21:u8=0
const c29:u8=0
const c51:u8=215
const c69:u8=189
const c79:u8=224
const c83:u8=45
const c94:u8=217
const c101:u8=49
const c130:u8=137
const c143:u8=0
const c148:u8=0
const c177:u8=203
const c182:u8=58
const c222:u8=2
const c243:u8=57
const c248:u8=172
const c254:u8=158
const c265:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ c12 c16) c21) (+ c29 (+ (- (- (+ 44 (- c51 (+ (+ 3 4) (- c69 (- (- c79 c83) (+ (- c94 (+ c101 149)) 0)))))) 64) (+ 22 c130)) (+ (+ c143 c148) (- (- (- 230 14) (+ (- c177 c182) (- (+ 6 21) (+ 9 (- 226 (- 228 (+ c222 18))))))) (- (+ c243 c248) c254)))))) c265))
"#);
}
#[test]
fn const_expr_48() {
    test_const_expr(r#"
const c6:u8=127
const c23:u8=0
static RESULT:u8
(= RESULT (- (+ c6 (+ (- 212 84) c23)) (- 16 16)))
"#);
}
#[test]
fn const_expr_49() {
    test_const_expr(r#"
const c17:u8=61
const c37:u8=87
const c56:u8=1
const c76:u8=0
const c81:u8=0
const c92:u8=0
const c103:u8=185
const c116:u8=161
const c128:u8=20
const c133:u8=60
const c147:u8=2
const c152:u8=10
const c172:u8=143
const c220:u8=50
const c227:u8=152
const c233:u8=171
const c254:u8=245
const c259:u8=8
const c272:u8=56
const c289:u8=36
const c294:u8=36
const c300:u8=4
const c309:u8=68
const c314:u8=0
const c326:u8=166
const c340:u8=10
const c351:u8=22
const c368:u8=244
const c376:u8=8
const c381:u8=0
const c388:u8=207
const c399:u8=12
const c420:u8=1
const c435:u8=36
const c440:u8=144
const c449:u8=45
const c468:u8=91
static RESULT:u8
(= RESULT (+ (+ 36 219) (- c17 (- 204 (+ (- (+ c37 (- (+ (+ (+ (+ c56 (+ 0 (+ (+ (+ 0 c76) c81) (+ (+ c92 (- 185 c103)) (- (- c116 80) (+ c128 c133)))))) (+ c147 c152)) (- 157 100)) c172) (- (+ 139 (- 106 (- (+ (- (+ (+ 19 (- 107 c220)) c227) c233) (+ (+ 19 (- (- c254 c259) (- 198 c272))) (- (- (+ c289 c294) c300) (+ c309 c314)))) (- c326 (- (+ (+ c340 (+ 11 c351)) 87) (- (- c368 (+ c376 c381)) c388)))))) c399))) (- (- 207 (+ c420 (- 187 (+ c435 c440)))) c449)) (+ 17 (+ 15 c468)))))))
"#);
}
#[test]
fn const_expr_50() {
    test_const_expr(r#"
const c15:u8=174
const c23:u8=107
const c32:u8=74
const c66:u8=84
const c80:u8=199
const c96:u8=98
const c102:u8=0
const c120:u8=25
const c138:u8=33
const c156:u8=12
const c161:u8=63
const c168:u8=48
static RESULT:u8
(= RESULT (- (+ 42 (+ (- c15 68) c23)) (- c32 (- (- (- (- (- (+ (+ (+ (+ 41 c66) (+ (- (- c80 135) (- 162 c96)) c102)) 125) 0) 5) c120) (+ (- (+ 10 c138) 38) 18)) (+ c156 c161)) c168))))
"#);
}
#[test]
fn const_expr_51() {
    test_const_expr(r#"
const c23:u8=104
const c29:u8=6
static RESULT:u8
(= RESULT (+ 85 (- (- 187 (- 115 c23)) c29)))
"#);
}
#[test]
fn const_expr_52() {
    test_const_expr(r#"
const c12:u8=71
const c16:u8=29
const c34:u8=251
const c55:u8=57
const c77:u8=29
const c91:u8=0
const c95:u8=0
const c101:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- c12 c16) (+ 213 (- (- c34 (- 242 (+ (- 128 c55) 144))) (+ (- 253 c77) 0)))) (+ c91 c95)) c101))
"#);
}
#[test]
fn const_expr_53() {
    test_const_expr(r#"
const c31:u8=64
const c38:u8=144
const c60:u8=160
const c70:u8=0
const c83:u8=124
const c92:u8=222
const c99:u8=1
const c121:u8=81
static RESULT:u8
(= RESULT (- (- (+ (+ (- 176 (+ (- (+ (- c31 (- c38 (+ 27 84))) (+ 32 c60)) 55) c70)) (- 158 c83)) (- c92 (+ c99 8))) 0) (- (- 220 c121) 139)))
"#);
}
#[test]
fn const_expr_54() {
    test_const_expr(r#"
const c15:u8=250
const c22:u8=14
const c30:u8=21
static RESULT:u8
(= RESULT (+ 36 (+ (- (- c15 (- c22 4)) c30) 0)))
"#);
}
#[test]
fn const_expr_55() {
    test_const_expr(r#"
const c16:u8=21
const c26:u8=254
const c40:u8=42
const c61:u8=223
const c83:u8=0
static RESULT:u8
(= RESULT (- (- (+ 127 (+ c16 (- (- c26 (- 215 (+ c40 169))) 143))) (- c61 (- 240 17))) (+ 0 c83)))
"#);
}
#[test]
fn const_expr_56() {
    test_const_expr(r#"
const c28:u8=138
const c45:u8=2
const c59:u8=142
static RESULT:u8
(= RESULT (+ (+ 6 (+ (+ 1 6) (- (+ (- c28 (- (- 171 (+ c45 16)) 38)) c59) (- 188 46)))) 219))
"#);
}
#[test]
fn const_expr_57() {
    test_const_expr(r#"
const c52:u8=27
const c57:u8=168
const c97:u8=34
const c107:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (- 143 (- (- 219 (+ 6 (- 212 (- (+ (+ 6 c52) c57) 16)))) 170)) 128) (+ 0 0)) (+ 0 (- c97 34))) c107))
"#);
}
#[test]
fn const_expr_58() {
    test_const_expr(r#"
const c36:u8=98
const c41:u8=8
const c55:u8=6
const c65:u8=79
const c75:u8=11
const c82:u8=84
const c106:u8=90
const c112:u8=105
const c121:u8=0
const c128:u8=42
const c143:u8=33
const c175:u8=132
const c206:u8=8
const c211:u8=3
const c252:u8=1
const c257:u8=3
const c274:u8=14
const c282:u8=168
const c297:u8=0
const c302:u8=0
const c310:u8=129
const c316:u8=208
const c345:u8=22
const c364:u8=73
const c369:u8=146
static RESULT:u8
(= RESULT (+ (- 128 (- 140 (+ 16 (+ (+ (- 106 c36) c41) (- (+ (+ c55 (+ (- c65 (- (+ c75 (- c82 25)) (- 19 (- (+ 29 c106) c112)))) c121)) c128) (- (- (+ c143 166) (+ (- (+ (+ (+ (- 132 c175) (+ 0 (+ 0 0))) 2) 12) (- c206 c211)) 18)) (- (- (+ (- (- (+ (+ (+ 1 (+ c252 c257)) (+ 3 (+ 6 c274))) c282) (+ 26 (+ c297 c302))) c310) c316) (+ (- 109 105) 28)) (+ c345 69)))))))) (+ c364 c369)))
"#);
}
#[test]
fn const_expr_59() {
    test_const_expr(r#"
const c18:u8=85
const c22:u8=170
const c30:u8=0
const c46:u8=122
const c53:u8=145
const c71:u8=235
const c84:u8=252
const c91:u8=19
const c128:u8=253
const c139:u8=0
const c155:u8=14
const c162:u8=108
const c185:u8=218
const c200:u8=65
const c209:u8=31
const c217:u8=135
const c229:u8=181
const c238:u8=0
const c244:u8=0
const c256:u8=252
const c264:u8=171
const c269:u8=161
const c323:u8=89
static RESULT:u8
(= RESULT (- (- (- (- (- (+ c18 c22) (+ c30 (+ (+ (+ (- c46 (- c53 23)) (- (- (- c71 (- (- (- c84 (+ c91 78)) 26) 87)) (- 186 (+ (- (- (- c128 0) (+ c139 0)) (- 249 c155)) c162))) 133)) 0) (- (- c185 (- (- 228 c200) (+ c209 (- c217 40)))) c229)))) c238) c244) (- (- c256 (- c264 c269)) 242)) (+ 0 (- (- 248 (- (+ 31 156) (+ 14 14))) c323))))
"#);
}
#[test]
fn const_expr_60() {
    test_const_expr(r#"
const c25:u8=95
const c32:u8=90
const c41:u8=2
const c52:u8=7
const c78:u8=157
static RESULT:u8
(= RESULT (+ (- (- (- (+ 254 (- (- c25 5) c32)) 1) c41) (+ (+ c52 (+ (- 130 122) 8)) (- c78 14))) 170))
"#);
}
#[test]
fn const_expr_61() {
    test_const_expr(r#"
const c3:u8=63
const c6:u8=192
static RESULT:u8
(= RESULT (+ c3 c6))
"#);
}
#[test]
fn const_expr_62() {
    test_const_expr(r#"
const c9:u8=51
const c12:u8=204
const c20:u8=23
const c44:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 c12) (- c20 (- 204 (+ 90 91)))) c44))
"#);
}
#[test]
fn const_expr_63() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 85 170))
"#);
}
#[test]
fn const_expr_64() {
    test_const_expr(r#"
const c9:u8=36
const c15:u8=54
const c31:u8=0
const c36:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 (+ c15 (+ 82 83))) c31) c36))
"#);
}
#[test]
fn const_expr_65() {
    test_const_expr(r#"
const c6:u8=192
static RESULT:u8
(= RESULT (+ 63 c6))
"#);
}
#[test]
fn const_expr_66() {
    test_const_expr(r#"
const c39:u8=1
const c49:u8=48
const c53:u8=98
const c61:u8=214
const c65:u8=70
const c75:u8=103
const c85:u8=11
const c92:u8=152
const c96:u8=119
const c124:u8=157
const c205:u8=217
const c210:u8=74
const c219:u8=185
const c228:u8=202
const c255:u8=96
const c260:u8=0
const c271:u8=85
const c282:u8=0
const c291:u8=0
const c305:u8=158
const c319:u8=59
const c324:u8=59
const c336:u8=248
const c358:u8=11
const c363:u8=23
const c374:u8=131
const c393:u8=29
const c398:u8=116
const c425:u8=18
const c448:u8=216
const c462:u8=87
const c479:u8=0
const c495:u8=190
const c515:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (- (- (+ (+ (- (+ (+ (+ (+ c39 (- (+ c49 c53) (- c61 c65))) (- c75 (+ (+ c85 (- c92 c96)) 44))) (+ (+ (- 160 (+ c124 (- 235 235))) (+ (- 139 (- 242 118)) (- (+ 123 123) (+ 41 205)))) (- 143 (- c205 c210)))) c219) (- c228 (- 181 (+ 39 (+ 23 (+ c255 c260)))))) c271) 128) c282) (+ c291 (- (- (- c305 (+ (+ (- c319 c324) (- (- c336 188) (+ (- (+ (+ c358 c363) 105) c374) (+ (- 171 (+ c393 c398)) (+ (- 229 226) (+ 5 c425)))))) 0)) 144) (- c448 (+ 28 (+ c462 87)))))) 0) c479) (- (- 193 c495) (- 241 238))) c515))
"#);
}
#[test]
fn const_expr_67() {
    test_const_expr(r#"
const c38:u8=80
const c48:u8=247
const c61:u8=5
const c65:u8=6
const c70:u8=44
const c92:u8=147
const c96:u8=146
const c114:u8=117
const c119:u8=118
const c149:u8=0
const c173:u8=8
const c198:u8=68
const c224:u8=5
const c229:u8=0
const c250:u8=14
const c255:u8=43
const c275:u8=6
const c289:u8=11
const c302:u8=0
const c307:u8=1
const c313:u8=8
const c324:u8=150
const c329:u8=13
const c336:u8=113
const c342:u8=51
const c357:u8=218
const c390:u8=0
const c421:u8=2
static RESULT:u8
(= RESULT (- (- (- (- (+ (- (- (- (+ 84 169) (- c38 (- (- c48 (- (+ (+ c61 c65) c70) 15)) (+ (+ (+ (- c92 c96) 9) 21) (- (+ c114 c119) (- 152 (+ (+ (+ 0 0) (+ c149 4)) (+ (- 104 103) c173)))))))) 21) 181) (+ c198 (+ 34 (+ (- (+ (+ (+ c224 c229) (- (+ (+ (- (+ c250 c255) 56) (+ 2 (+ 1 c275))) (+ 10 c289)) (+ (+ c302 c307) c313))) (- c324 c329)) c336) c342)))) (- (- c357 214) (- (- 105 30) 71))) 0) c390) (+ 0 (- 3 (+ (- 179 178) c421)))))
"#);
}
#[test]
fn const_expr_68() {
    test_const_expr(r#"
const c3:u8=63
static RESULT:u8
(= RESULT (+ c3 192))
"#);
}
#[test]
fn const_expr_69() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 63 192))
"#);
}
#[test]
fn const_expr_70() {
    test_const_expr(r#"
const c23:u8=186
const c27:u8=183
const c46:u8=0
const c65:u8=42
const c72:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ (+ (+ 2 (- c23 c27)) (- 141 110)) c46) (+ 109 (- 152 c65))) c72) 0))
"#);
}
#[test]
fn const_expr_71() {
    test_const_expr(r#"
const c27:u8=63
const c44:u8=38
const c74:u8=50
const c82:u8=0
const c86:u8=0
const c93:u8=36
const c118:u8=155
const c130:u8=139
const c143:u8=205
const c152:u8=62
const c159:u8=34
const c191:u8=0
const c209:u8=0
const c226:u8=159
const c238:u8=20
const c245:u8=24
const c281:u8=217
const c286:u8=5
const c305:u8=0
const c313:u8=0
const c330:u8=181
const c342:u8=6
const c363:u8=2
const c402:u8=108
const c409:u8=114
const c426:u8=19
const c449:u8=0
const c474:u8=176
const c497:u8=0
const c534:u8=0
const c554:u8=11
const c565:u8=22
const c581:u8=89
const c592:u8=126
const c597:u8=108
const c605:u8=188
const c626:u8=9
const c647:u8=228
const c652:u8=38
const c669:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (- (- (- (- (+ c27 192) (+ 0 (- c44 (+ (+ 7 (- (+ 36 (+ (- 50 c74) (+ c82 c86))) c93)) (- (- (- (+ (- 235 c118) (+ 23 c130)) (- (- c143 70) c152)) c159) (- 216 112)))))) 0) (+ (+ c191 0) 0)) (+ (+ c209 (- (+ (+ (- c226 (- 156 c238)) c245) (+ 27 162)) (- (- (- (+ 42 (- c281 c286)) 1) (+ (+ (+ c305 (+ c313 (+ 0 (- 181 c330)))) 0) c342)) (+ (+ (+ 0 0) c363) (+ (- 182 (+ 30 (+ (- 252 (+ 107 c402)) c409))) (+ (- (+ c426 115) 132) (+ (+ 1 c449) (+ (- (+ 177 0) (+ c474 0)) 4)))))))) 0)) c497) (- 164 164)) (+ 0 0)) (+ (+ (+ c534 0) (- (- (+ (+ c554 (+ (- c565 (+ 0 (- 93 c581))) (- c592 c597))) c605) (- 224 (+ (+ 8 c626) 51))) (+ (- (- c647 c652) 171) 60))) c669)))
"#);
}
#[test]
fn const_expr_72() {
    test_const_expr(r#"
const c6:u8=63
const c9:u8=192
static RESULT:u8
(= RESULT (- (+ c6 c9) 0))
"#);
}
#[test]
fn const_expr_73() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ (+ (- (+ 7 40) 37) 53) 192))
"#);
}
#[test]
fn const_expr_74() {
    test_const_expr(r#"
const c22:u8=243
const c26:u8=0
const c41:u8=128
const c64:u8=3
const c68:u8=0
static RESULT:u8
(= RESULT (- (+ (- 236 (+ (- (+ c22 c26) 228) 94)) c41) (- (- (+ 9 49) (+ c64 c68)) 55)))
"#);
}
#[test]
fn const_expr_75() {
    test_const_expr(r#"
const c13:u8=170
static RESULT:u8
(= RESULT (+ (+ 17 68) c13))
"#);
}
#[test]
fn const_expr_76() {
    test_const_expr(r#"
const c26:u8=190
static RESULT:u8
(= RESULT (+ 36 (- 222 (- 240 (+ 47 c26)))))
"#);
}
#[test]
fn const_expr_77() {
    test_const_expr(r#"
const c6:u8=213
static RESULT:u8
(= RESULT (+ 42 c6))
"#);
}
#[test]
fn const_expr_78() {
    test_const_expr(r#"
const c9:u8=63
const c22:u8=6
const c44:u8=82
const c57:u8=254
const c61:u8=5
const c72:u8=169
const c85:u8=70
const c110:u8=47
const c127:u8=144
const c137:u8=209
const c157:u8=151
const c193:u8=197
const c199:u8=218
const c211:u8=4
const c222:u8=138
const c235:u8=161
const c243:u8=20
const c259:u8=36
const c270:u8=12
const c298:u8=106
const c313:u8=218
const c336:u8=0
const c345:u8=10
const c350:u8=0
const c357:u8=38
const c365:u8=25
const c384:u8=205
const c396:u8=4
const c434:u8=3
const c440:u8=148
const c449:u8=155
const c484:u8=124
const c514:u8=0
const c551:u8=241
static RESULT:u8
(= RESULT (- (- (+ c9 (- 205 (+ c22 (- 77 (+ (+ (- (- c44 (- (- (- c57 c61) (+ (- c72 (+ (- 96 c85) 132)) 48)) (- (- (+ c110 (+ (+ 48 0) c127)) (- c137 (+ (- 215 178) c157))) 58))) 50) (+ (- (+ (- (+ 39 c193) c199) (+ (+ c211 (+ (- c222 135) (- c235 (+ c243 125)))) (+ c259 (+ (+ c270 0) (+ (+ 12 12) 0))))) c298) (- (- (- c313 (- 233 (+ (+ (+ 9 c336) (+ c345 c350)) c357))) c365) (+ (+ (- 205 c384) (- (+ c396 (- 146 (+ 34 103))) (- (- (- 251 c434) c440) (- c449 (- 170 103))))) (+ 1 (- (+ (+ c484 (- 251 251)) 125) (+ 242 c514))))))) 53))))) (- (- 251 (- 241 c551)) 251)) 0))
"#);
}
#[test]
fn const_expr_79() {
    test_const_expr(r#"
const c15:u8=64
const c20:u8=128
static RESULT:u8
(= RESULT (+ (+ (+ 9 54) c15) c20))
"#);
}
#[test]
fn const_expr_80() {
    test_const_expr(r#"
const c11:u8=16
const c16:u8=18
const c27:u8=183
static RESULT:u8
(= RESULT (+ (+ (+ 2 c11) c16) (+ 36 c27)))
"#);
}
#[test]
fn const_expr_81() {
    test_const_expr(r#"
const c3:u8=63
const c12:u8=153
const c29:u8=12
const c50:u8=190
const c54:u8=186
const c63:u8=93
const c73:u8=70
const c88:u8=29
const c92:u8=146
const c100:u8=11
const c116:u8=15
static RESULT:u8
(= RESULT (+ c3 (+ (- c12 (- (- (- 254 c29) 138) (+ (+ 1 (- c50 c54)) (- c63 (+ 13 c73))))) (- (+ c88 c92) (+ c100 (+ 18 (+ 3 c116)))))))
"#);
}
#[test]
fn const_expr_82() {
    test_const_expr(r#"
const c22:u8=196
const c36:u8=5
const c57:u8=67
const c67:u8=11
const c71:u8=34
const c82:u8=180
const c86:u8=53
const c91:u8=36
const c98:u8=203
const c111:u8=0
const c124:u8=93
static RESULT:u8
(= RESULT (+ 127 (- (+ (+ 8 40) c22) (+ (+ (+ c36 (+ (+ 0 0) (- (+ c57 (+ (+ c67 c71) (- (- c82 c86) c91))) c98))) (+ (+ c111 2) 16)) c124))))
"#);
}
#[test]
fn const_expr_83() {
    test_const_expr(r#"
const c9:u8=36
const c19:u8=22
const c28:u8=0
const c53:u8=1
const c79:u8=3
const c89:u8=61
const c109:u8=5
const c117:u8=6
const c122:u8=26
static RESULT:u8
(= RESULT (- (- (+ c9 (- 241 c19)) (+ c28 (- (- (- 249 (+ (+ 0 c53) 10)) 36) (+ (+ 13 (+ c79 (- (+ c89 (- 148 148)) (+ c109 (+ c117 c122))))) 162)))) 0))
"#);
}
#[test]
fn const_expr_84() {
    test_const_expr(r#"
const c9:u8=85
static RESULT:u8
(= RESULT (- (+ (+ c9 170) 0) 0))
"#);
}
#[test]
fn const_expr_85() {
    test_const_expr(r#"
const c6:u8=147
static RESULT:u8
(= RESULT (+ (- c6 20) 128))
"#);
}
#[test]
fn const_expr_86() {
    test_const_expr(r#"
const c9:u8=127
const c15:u8=25
const c19:u8=103
const c40:u8=48
const c44:u8=97
const c61:u8=108
const c88:u8=35
const c99:u8=78
const c136:u8=67
const c142:u8=97
const c148:u8=114
const c161:u8=22
const c178:u8=237
const c195:u8=203
const c205:u8=6
const c213:u8=77
const c218:u8=65
const c249:u8=9
const c255:u8=34
const c291:u8=186
const c302:u8=121
const c322:u8=18
const c336:u8=2
const c349:u8=241
const c390:u8=175
const c399:u8=149
const c416:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 (+ c15 c19)) (+ (- (+ 28 (+ c40 c44)) (+ (+ 6 (- c61 (+ (- (+ 125 (- 178 (+ c88 (- 221 c99)))) (+ (- 164 (+ (- (+ (+ 16 51) c136) c142) c148)) (- (+ c161 (+ (+ (+ (- c178 237) (- 207 c195)) (+ c205 (- c213 c218))) 68)) (+ (+ (- 232 230) c249) c255)))) (- (+ (- 75 (+ (- (- 52 (- c291 (+ 30 c302))) (+ (- 64 (+ c322 38)) 9)) c336)) (- (- c349 (+ (+ (- 76 71) 26) 31)) (+ 32 0))) c390)))) c399)) (+ 0 0))) c416))
"#);
}
#[test]
fn const_expr_87() {
    test_const_expr(r#"
const c3:u8=85
static RESULT:u8
(= RESULT (+ c3 (+ 28 142)))
"#);
}
#[test]
fn const_expr_88() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (- (+ (- 75 33) 213) 0))
"#);
}
#[test]
fn const_expr_89() {
    test_const_expr(r#"
const c15:u8=51
const c26:u8=26
const c35:u8=0
const c43:u8=0
static RESULT:u8
(= RESULT (+ (- (- (- (+ c15 (- 230 c26)) 0) c35) 0) c43))
"#);
}
#[test]
fn const_expr_90() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (- (+ 51 204) 0))
"#);
}
#[test]
fn const_expr_91() {
    test_const_expr(r#"
const c6:u8=63
const c9:u8=192
const c25:u8=99
const c44:u8=60
const c57:u8=240
const c61:u8=9
const c78:u8=192
const c92:u8=153
const c107:u8=12
const c152:u8=184
const c201:u8=25
const c208:u8=28
const c227:u8=147
const c236:u8=16
const c241:u8=33
const c251:u8=61
const c273:u8=131
static RESULT:u8
(= RESULT (- (+ c6 c9) (+ (- 99 (+ c25 (+ (- (- (- (+ c44 (- (- (- c57 c61) (- 63 (- (- c78 (+ (- 153 c92) 3)) (+ (+ c107 12) 121)))) (+ 5 27))) 5) 161) (- (- (- c152 28) (+ (- (+ (+ 1 (+ (- (- (- 233 8) (- 221 c201)) c208) (- (+ (- 155 c227) (+ c236 c241)) (- c251 9)))) 16) 19) (- c273 103))) (- (+ 166 0) 116))) 0))) 0)))
"#);
}
#[test]
fn const_expr_92() {
    test_const_expr(r#"
const c27:u8=208
const c85:u8=13
const c93:u8=12
const c123:u8=107
const c135:u8=14
const c142:u8=16
const c166:u8=61
const c191:u8=148
const c203:u8=76
const c228:u8=210
const c243:u8=7
const c251:u8=9
const c256:u8=20
const c339:u8=20
const c348:u8=7
const c362:u8=144
const c367:u8=143
static RESULT:u8
(= RESULT (+ (- (- (+ (- (+ (- (+ (- c27 152) (+ (- 68 12) (- (+ (- (- (- 249 4) (- 71 (+ (+ 3 c85) (+ c93 39)))) 4) (- (- 100 (+ (- c123 (- 117 c135)) c142)) (- 213 133))) (+ c166 (+ 8 (+ (- (- (+ 49 c191) (+ 38 c203)) (+ (- 148 138) (- c228 155))) (+ c243 (+ c251 c256)))))))) (+ 35 140)) (+ 51 153)) 0) 0) 0) (+ 0 0)) (+ 0 (+ (- 147 147) (- (+ 9 c339) (+ c348 (+ (+ (- c362 c367) 3) 18)))))))
"#);
}
#[test]
fn const_expr_93() {
    test_const_expr(r#"
const c9:u8=132
const c34:u8=217
const c48:u8=117
const c79:u8=31
const c86:u8=227
const c95:u8=64
const c116:u8=102
const c129:u8=22
const c134:u8=115
const c153:u8=15
const c162:u8=0
const c187:u8=14
const c196:u8=169
const c202:u8=198
const c212:u8=246
const c217:u8=238
const c227:u8=128
const c248:u8=35
const c271:u8=234
const c289:u8=50
const c306:u8=168
const c318:u8=79
const c338:u8=14
const c354:u8=192
const c372:u8=243
const c387:u8=196
const c399:u8=156
const c427:u8=251
const c434:u8=2
const c449:u8=190
const c490:u8=20
const c495:u8=124
const c508:u8=57
const c519:u8=0
const c527:u8=24
const c553:u8=24
const c558:u8=144
const c564:u8=166
const c579:u8=31
const c587:u8=0
const c605:u8=94
const c649:u8=3
const c668:u8=1
const c685:u8=3
const c711:u8=0
const c729:u8=0
const c742:u8=80
const c757:u8=25
const c763:u8=65
const c773:u8=251
const c778:u8=9
const c786:u8=0
const c792:u8=0
const c832:u8=100
const c840:u8=202
const c873:u8=31
const c884:u8=1
const c896:u8=166
const c906:u8=0
const c935:u8=148
const c946:u8=0
const c959:u8=63
const c964:u8=63
const c998:u8=174
const c1012:u8=14
const c1028:u8=113
const c1038:u8=70
const c1047:u8=122
const c1053:u8=65
const c1100:u8=47
const c1117:u8=254
const c1123:u8=2
const c1150:u8=69
const c1157:u8=85
static RESULT:u8
(= RESULT (- (+ (- c9 (- 15 (- (- 242 (+ (- c34 (+ (- 146 c48) (+ (- 229 (- 235 (- (- (+ c79 (- c86 35)) c95) (+ (- 206 (+ 33 c116)) (- (+ c129 c134) 65))))) (+ 5 c153)))) c162)) (- (+ 27 (- (+ (+ c187 70) c196) c202)) (- c212 c217))))) c227) (- (- (- (+ (+ c248 (+ (+ (- (+ (- (- c271 (+ (- 152 (+ c289 100)) (+ (- c306 (- 245 c318)) (+ (- 32 30) c338)))) 35) (- c354 (- (- 246 (- c372 (+ (- 230 c387) (+ 52 c399)))) (+ (- (+ 15 (- 251 c427)) c434) (- (+ (- c449 168) 92) (+ 37 (- 243 206))))))) (+ c490 c495)) (+ (- c508 (+ (+ c519 3) c527)) (- (- 252 (+ (- (+ c553 c558) c564) (- 37 (+ c579 (+ c587 (+ 0 0)))))) c605))) (+ (+ (- (+ (+ (- 44 (+ 7 37)) (+ 0 c649)) 3) (+ (- (+ c668 10) (+ (+ 1 c685) 5)) 4)) (+ (+ (+ (+ c711 (+ (- 50 50) c729)) (- (+ c742 (- (- 252 c757) c763)) (- c773 c778))) c786) c792)) 0))) (- 231 (+ (+ (+ (+ (- (+ (- c832 (- c840 (- (- 247 45) 78))) 134) (+ c873 125)) c884) (- (+ c896 (+ 0 c906)) (+ (+ (+ (+ (+ (- 148 c935) (+ 0 c946)) (+ (- c959 c964) (+ 2 0))) 6) 18) 130))) 46) c998))) 5) (+ c1012 (+ (- 115 c1028) (- c1038 (- c1047 c1053))))) (- 234 (- (+ (- 127 (- (- 246 (+ 15 c1100)) (- (- (- c1117 c1123) 100) (- 193 132)))) c1150) c1157)))))
"#);
}
#[test]
fn const_expr_94() {
    test_const_expr(r#"
const c15:u8=48
const c35:u8=0
const c55:u8=211
const c60:u8=57
const c80:u8=109
const c117:u8=230
const c134:u8=123
const c158:u8=63
const c164:u8=49
const c178:u8=0
static RESULT:u8
(= RESULT (+ (+ (- (- (+ c15 193) (+ 1 (+ (+ c35 (- (+ (+ (- 230 c55) c60) 76) 152)) 4))) c80) (+ (+ 32 (- (- (- 232 41) (+ (- c117 223) (- 171 c134))) (- 133 (- (+ 15 c158) c164)))) 64)) c178))
"#);
}
#[test]
fn const_expr_95() {
    test_const_expr(r#"
const c6:u8=51
const c9:u8=204
const c13:u8=0
static RESULT:u8
(= RESULT (- (+ c6 c9) c13))
"#);
}
#[test]
fn const_expr_96() {
    test_const_expr(r#"
const c39:u8=114
const c56:u8=251
const c60:u8=12
const c84:u8=190
const c120:u8=239
const c128:u8=249
const c133:u8=191
const c140:u8=133
const c151:u8=42
const c185:u8=164
const c222:u8=36
const c244:u8=88
const c256:u8=83
const c266:u8=0
const c291:u8=78
const c302:u8=248
const c310:u8=37
const c315:u8=189
const c322:u8=134
const c332:u8=120
const c353:u8=64
const c381:u8=206
const c389:u8=210
const c394:u8=29
const c402:u8=2
const c418:u8=138
const c423:u8=49
const c434:u8=217
const c439:u8=90
const c455:u8=56
const c460:u8=112
const c471:u8=35
const c476:u8=140
static RESULT:u8
(= RESULT (- (- (+ (+ 127 (- (- 138 (- 136 (- (- c39 11) (- (- (- c56 c60) (- 232 (- (+ 56 (- c84 (- 102 82))) 5))) 171)))) (- (- c120 (- c128 c133)) c140))) (+ c151 86)) (+ (+ (+ (- 238 238) (- c185 (+ (+ (- (+ 50 (- 101 101)) 45) c222) (+ (- (+ 54 54) c244) (+ 20 c256))))) c266) (- (- 195 (- (- (+ c291 (+ (- c302 (+ c310 c315)) c322)) (- c332 (+ (+ (- 97 87) c353) (+ (- (- (- (- 253 (- c381 (- c389 c394))) c402) (- 188 (- c418 c423))) (- c434 c439)) 0)))) (+ c455 c460))) (+ c471 c476)))) 0))
"#);
}
#[test]
fn const_expr_97() {
    test_const_expr(r#"
const c12:u8=71
const c28:u8=116
const c32:u8=67
const c37:u8=49
const c68:u8=2
const c82:u8=26
const c103:u8=41
const c118:u8=0
const c126:u8=0
const c131:u8=0
const c139:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- c12 (- (- (+ (- c28 c32) c37) (+ 7 (+ 7 (- (- (- 254 1) c68) (+ (- 98 c82) (- 233 89)))))) c103)) 192) (+ c118 (+ c126 c131))) c139))
"#);
}
#[test]
fn const_expr_98() {
    test_const_expr(r#"
const c43:u8=15
const c54:u8=140
const c83:u8=170
const c102:u8=151
const c107:u8=35
const c116:u8=86
const c121:u8=67
const c150:u8=162
const c165:u8=0
const c170:u8=0
const c180:u8=0
const c199:u8=14
static RESULT:u8
(= RESULT (- (- (- (- (- (+ (+ (+ (- (- 244 (+ 22 (+ c43 (- 236 c54)))) 94) 68) (- 157 157)) c83) (+ 0 (- (- (- c102 c107) (- c116 c121)) (+ 32 65)))) (- (+ 32 c150) 194)) (+ c165 c170)) (+ c180 0)) (- 104 (+ c199 90))))
"#);
}
#[test]
fn const_expr_99() {
    test_const_expr(r#"
const c9:u8=253
const c24:u8=0
const c71:u8=199
const c95:u8=161
const c100:u8=26
const c109:u8=233
const c154:u8=84
const c174:u8=254
const c182:u8=1
const c194:u8=176
const c199:u8=148
const c215:u8=142
const c227:u8=34
const c232:u8=171
const c241:u8=242
const c257:u8=38
const c265:u8=31
const c276:u8=22
const c281:u8=132
const c303:u8=205
const c314:u8=55
const c323:u8=12
const c333:u8=112
const c338:u8=83
const c348:u8=42
const c356:u8=225
const c386:u8=11
const c410:u8=130
const c427:u8=3
const c437:u8=18
static RESULT:u8
(= RESULT (+ (- (- c9 (+ (+ (+ (+ c24 0) (- 1 1)) (+ (+ 0 (+ (+ 0 0) 0)) 0)) 3)) c71) (- (- (+ (- (- 222 c95) c100) (- c109 (+ (- 58 (- (+ 27 110) 84)) (- (- (+ (- c154 (+ (- (- (- (- c174 (+ c182 3)) (- c194 c199)) (- (+ 23 c215) (- (+ c227 c232) (- c241 190)))) (+ c257 (+ c265 (- (+ c276 c281) 30)))) (- (- (- c303 (+ (- c314 52) c323)) (- c333 c338)) (+ c348 (- c356 (- 143 3)))))) 99) (+ 10 c386)) (- (+ (- 110 84) c410) 61))))) (+ c427 22)) c437)))
"#);
}
#[test]
fn const_expr_100() {
    test_const_expr(r#"
const c16:u8=162
const c37:u8=95
const c75:u8=132
const c79:u8=66
const c101:u8=0
const c106:u8=2
const c116:u8=224
const c121:u8=213
const c130:u8=20
const c151:u8=0
const c159:u8=0
const c178:u8=8
const c200:u8=202
const c208:u8=239
const c225:u8=243
const c241:u8=0
const c261:u8=153
const c313:u8=134
const c318:u8=124
const c337:u8=23
const c342:u8=24
const c348:u8=141
const c366:u8=76
const c371:u8=51
const c383:u8=160
const c428:u8=136
const c442:u8=253
const c454:u8=0
const c462:u8=248
const c496:u8=26
const c503:u8=222
const c511:u8=14
static RESULT:u8
(= RESULT (- (+ (- 211 (- c16 78)) (- (- (+ (- c37 74) 129) (- 222 (- (- (+ 33 (+ (- c75 c79) (- 240 106))) (+ c101 c106)) (- c116 c121)))) c130)) (+ (- 6 (+ (+ c151 (+ c159 (+ 1 (- 34 (+ c178 26))))) (+ (+ (- c200 (- c208 (+ (- (- (- c225 (+ (+ (+ 1 c241) (+ (+ 0 1) (- c261 149))) (- (- 246 (- 91 27)) 182))) (+ 12 (+ (- c313 c318) (- 239 (+ (+ c337 c342) c348))))) 152) (- c366 c371)))) (- c383 (+ 39 120))) (- (+ (- (- 251 (+ 0 (+ (- c428 (+ 22 (- c442 139))) c454))) c462) (+ 7 (- (+ (+ 76 154) (- 26 c496)) c503))) c511)))) 0)))
"#);
}
#[test]
fn const_expr_101() {
    test_const_expr(r#"
const c3:u8=51
const c6:u8=204
static RESULT:u8
(= RESULT (+ c3 c6))
"#);
}
#[test]
fn const_expr_102() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (- (+ 127 128) 0))
"#);
}
#[test]
fn const_expr_103() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 36 (+ (+ 31 0) (- (- 227 18) 21))))
"#);
}
#[test]
fn const_expr_104() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (- (+ (- 228 (+ 96 96)) 219) 0))
"#);
}
#[test]
fn const_expr_105() {
    test_const_expr(r#"
const c18:u8=245
const c22:u8=101
const c27:u8=119
const c65:u8=32
const c81:u8=20
const c95:u8=36
const c106:u8=82
const c126:u8=239
const c137:u8=187
const c150:u8=181
const c155:u8=21
const c169:u8=98
const c192:u8=22
const c211:u8=58
const c217:u8=70
const c226:u8=205
const c259:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ (- (- c18 c22) c27) (+ (+ (- 223 (- (- 248 (+ 10 0)) c65)) 85) (- (+ c81 (- (- 202 c95) (- (+ c106 (+ 54 (- (+ (- c126 (+ (- c137 147) (- c150 c155))) (+ 19 c169)) (- (+ (- (- 129 c192) (- 253 204)) c211) c217)))) c226))) (- 229 84)))) 128) 0) (+ c259 0)))
"#);
}
#[test]
fn const_expr_106() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 63 192))
"#);
}
#[test]
fn const_expr_107() {
    test_const_expr(r#"
const c29:u8=44
const c43:u8=32
static RESULT:u8
(= RESULT (- (- (+ (- 254 (- 176 (- 93 c29))) (- 160 c43)) 0) 0))
"#);
}
#[test]
fn const_expr_108() {
    test_const_expr(r#"
const c9:u8=10
const c12:u8=41
const c47:u8=202
const c64:u8=9
const c71:u8=19
const c75:u8=0
const c87:u8=166
const c92:u8=99
const c102:u8=2
const c111:u8=1
const c116:u8=5
const c140:u8=50
const c152:u8=222
const c172:u8=27
const c180:u8=108
const c203:u8=10
const c235:u8=92
const c246:u8=27
const c268:u8=213
const c278:u8=16
const c306:u8=28
const c349:u8=124
const c359:u8=32
const c386:u8=7
const c392:u8=9
const c398:u8=75
const c404:u8=25
const c413:u8=71
const c418:u8=20
static RESULT:u8
(= RESULT (+ (+ (+ c9 c12) (+ 51 (- (- (- (- (+ (- (+ (- c47 (+ (- 101 (+ c64 (+ c71 c75))) 74)) c87) c92) 122) c102) (+ c111 c116)) (+ 23 23)) (+ (- c140 (- 254 c152)) (- 154 (+ (+ c172 0) c180)))))) (- (- (+ (+ c203 11) (+ (+ (- 200 (- 245 (- c235 42))) c246) (- (- 159 (+ (- c268 205) c278)) (- 168 132)))) (- (+ c306 114) (+ (+ (- (- 218 (+ (+ 12 0) 77)) c349) 11) c359))) (- (- (- (- (- 225 c386) c392) c398) c404) (- c413 c418)))))
"#);
}
#[test]
fn const_expr_109() {
    test_const_expr(r#"
const c47:u8=238
const c55:u8=51
const c63:u8=50
const c74:u8=193
const c95:u8=39
const c99:u8=195
const c104:u8=214
const c139:u8=4
const c147:u8=27
const c152:u8=27
const c170:u8=10
const c202:u8=0
const c220:u8=41
const c229:u8=0
const c261:u8=246
const c269:u8=203
const c318:u8=27
const c336:u8=2
const c347:u8=230
const c361:u8=40
const c370:u8=86
const c378:u8=149
const c389:u8=183
const c418:u8=24
const c423:u8=120
const c438:u8=63
const c443:u8=63
const c463:u8=227
const c473:u8=54
const c494:u8=2
const c502:u8=249
const c526:u8=53
const c535:u8=118
const c546:u8=250
const c566:u8=254
const c574:u8=153
const c582:u8=227
const c597:u8=13
const c608:u8=26
const c623:u8=0
const c678:u8=33
const c729:u8=105
const c738:u8=25
const c759:u8=12
const c767:u8=10
const c809:u8=243
const c814:u8=234
const c820:u8=9
const c829:u8=0
const c838:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (+ (+ 2 (+ (+ 0 (- (- 216 (- (- (- c47 16) c55) (+ c63 101))) c74)) (- 92 (+ (- (+ c95 c99) c104) (+ 12 (- 251 (- (- 223 (+ (+ c139 (- c147 c152)) (+ (+ 0 2) c170))) (+ (+ (- (+ 98 99) 197) c202) (+ (+ (- 41 c220) (+ c229 0)) (+ 2 2)))))))))) (- (- c261 (- c269 202)) 177)) (- (- (+ (+ 9 (- (- (- (+ 26 (+ c318 (- 122 (+ (+ c336 6) (- c347 197))))) c361) (- c370 (- c378 (+ (- c389 170) (+ (- (- 221 8) (+ c418 c423)) (+ 0 (- c438 c443))))))) 35)) (- c463 30)) c473) (+ (- (- (+ (+ c494 (- c502 (- 239 2))) 43) 0) c526) (- c535 (- (- c546 0) (- (- (- (- c566 (- c574 (- c582 (- (- 121 c597) (+ 8 c608))))) (+ 0 c623)) 2) (- 132 (+ 7 (+ (- 153 146) 16))))))))) (- (+ c678 (- (+ 31 190) 22)) (- (+ (+ (- 104 (+ 42 42)) c729) (+ c738 (+ (- 134 (- (+ c759 (+ c767 (- 214 (- 246 94)))) 0)) 50))) (+ (- c809 c814) c820)))) c829) (+ c838 (+ 0 0))))
"#);
}
#[test]
fn const_expr_110() {
    test_const_expr(r#"
const c18:u8=7
const c22:u8=44
const c36:u8=132
const c40:u8=10
const c48:u8=123
const c52:u8=0
const c61:u8=10
const c65:u8=31
const c84:u8=44
const c117:u8=108
const c136:u8=45
const c141:u8=0
const c152:u8=82
const c167:u8=20
const c175:u8=20
const c206:u8=0
const c217:u8=187
const c241:u8=75
const c250:u8=7
const c261:u8=192
const c292:u8=106
const c323:u8=252
const c351:u8=54
const c367:u8=111
const c401:u8=16
const c409:u8=116
const c428:u8=238
const c433:u8=0
const c442:u8=149
const c447:u8=140
const c471:u8=6
const c530:u8=34
const c541:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (+ (+ c18 c22) (- (+ (- c36 c40) (+ c48 c52)) (+ c61 c65))) (- (+ (+ (+ c84 (+ (- 192 (+ 27 (+ 19 119))) c117)) (- (- (+ (+ c136 c141) 180) c152) 143)) (- c167 (+ c175 0))) 179)) 0) 0) (+ (+ (+ c206 0) (- c217 187)) (+ (- (- 118 c241) (+ c250 (- (- c261 (- (- (- (- 249 (+ (- 120 c292) 15)) 66) 22) (- (- (- (- c323 6) (+ 7 (+ (+ (- (+ 26 c351) 80) 0) (- c367 111)))) 4) (- (+ (+ 41 (- (+ c401 (- c409 17)) (- (- (- c428 c433) (- c442 c447)) 198))) (+ 126 (- c471 6))) (- (+ (- 232 180) 105) (+ (- 30 (- 119 (+ (- 138 c530) (+ 0 c541)))) 0)))))) (+ 50 (+ 50 50))))) 0))))
"#);
}
#[test]
fn const_expr_111() {
    test_const_expr(r#"
const c24:u8=29
const c39:u8=234
const c58:u8=19
const c73:u8=34
const c87:u8=253
const c91:u8=4
const c106:u8=182
const c126:u8=193
const c131:u8=193
const c144:u8=190
const c152:u8=190
static RESULT:u8
(= RESULT (- (- (- (- (+ (+ (- (+ c24 87) 65) (- c39 (+ 7 (- (- 243 c58) (- (- 241 c73) (- (- (- c87 c91) (+ 61 0)) c106)))))) 0) 0) (- c126 c131)) (- (+ c144 0) c152)) 0))
"#);
}
#[test]
fn const_expr_112() {
    test_const_expr(r#"
const c12:u8=2
const c19:u8=4
const c30:u8=35
const c39:u8=170
const c47:u8=0
const c51:u8=0
static RESULT:u8
(= RESULT (- (+ (+ (+ c12 (+ c19 8)) (+ c30 36)) c39) (+ c47 c51)))
"#);
}
#[test]
fn const_expr_113() {
    test_const_expr(r#"
const c9:u8=100
const c18:u8=130
const c22:u8=112
const c27:u8=19
const c41:u8=0
const c45:u8=0
static RESULT:u8
(= RESULT (- (+ (- c9 (+ (- c18 c22) c27)) 192) (+ c41 c45)))
"#);
}
#[test]
fn const_expr_114() {
    test_const_expr(r#"
const c17:u8=90
const c26:u8=201
const c39:u8=129
const c69:u8=0
const c80:u8=21
const c122:u8=205
const c145:u8=216
const c157:u8=0
const c177:u8=21
const c185:u8=32
const c207:u8=67
const c236:u8=172
const c241:u8=84
const c250:u8=9
const c255:u8=47
const c265:u8=21
const c276:u8=163
const c291:u8=3
const c305:u8=246
const c319:u8=80
const c324:u8=10
const c330:u8=48
const c342:u8=11
const c356:u8=114
const c383:u8=0
const c393:u8=197
const c399:u8=35
static RESULT:u8
(= RESULT (+ (+ 127 (- 218 c17)) (- c26 (- (+ (- c39 (+ (+ (- (+ (+ (- 101 86) c69) (+ 10 c80)) (- (- 204 (+ 10 11)) 159)) (+ (+ (- c122 (+ 68 137)) (+ (- c145 (+ 216 c157)) (+ (- 152 (+ c177 (+ c185 (- (+ 33 133) (+ c207 0))))) (+ 0 (- (+ (- (- c236 c241) (+ c250 c255)) (+ c265 110)) c276))))) (- 3 c291))) (- (- c305 (+ (- (- c319 c324) c330) (+ (+ c342 (+ 5 6)) c356))) (+ (- 69 67) (+ 18 c383))))) c393) c399))))
"#);
}
#[test]
fn const_expr_115() {
    test_const_expr(r#"
const c31:u8=9
const c42:u8=155
const c50:u8=73
const c64:u8=202
const c74:u8=39
const c81:u8=41
const c92:u8=249
const c96:u8=18
const c106:u8=0
const c112:u8=0
const c139:u8=102
const c150:u8=201
const c166:u8=155
const c212:u8=232
const c265:u8=13
const c273:u8=16
const c289:u8=250
const c297:u8=142
const c302:u8=24
const c326:u8=27
static RESULT:u8
(= RESULT (+ (- (- (+ (+ (- (+ (- (- 109 c31) (- (- c42 14) c50)) 128) (- c64 (+ 13 c74))) c81) (- (- c92 c96) 27)) c106) c112) (- (+ 25 (- (+ (+ (- c139 (+ (- c150 191) (+ (- c166 (+ 41 (+ (- (- (+ 56 (- 243 75)) 140) (- c212 164)) (- 162 (- (+ 16 82) 2))))) 32))) 0) (- (+ c265 (+ c273 51)) (- (- c289 (- c297 c302)) 81))) (+ (+ 1 4) c326))) 50)))
"#);
}
#[test]
fn const_expr_116() {
    test_const_expr(r#"
const c6:u8=63
const c12:u8=48
const c16:u8=144
static RESULT:u8
(= RESULT (- (+ c6 (+ c12 c16)) 0))
"#);
}
#[test]
fn const_expr_117() {
    test_const_expr(r#"
const c9:u8=0
const c16:u8=36
const c39:u8=147
const c43:u8=138
const c48:u8=39
static RESULT:u8
(= RESULT (+ (+ 36 c9) (+ c16 (+ 91 (- 140 (+ (- c39 c43) c48))))))
"#);
}
#[test]
fn const_expr_118() {
    test_const_expr(r#"
const c12:u8=151
const c28:u8=6
const c35:u8=15
const c39:u8=15
const c45:u8=35
const c60:u8=252
const c71:u8=53
const c80:u8=210
const c90:u8=57
const c94:u8=37
const c106:u8=0
const c115:u8=57
static RESULT:u8
(= RESULT (- (- (+ (- c12 (+ (+ (- (+ c28 (+ c35 c39)) c45) (+ (- 253 c60) (- (+ c71 159) c80))) (- c90 c94))) 128) c106) (- c115 57)))
"#);
}
#[test]
fn const_expr_119() {
    test_const_expr(r#"
const c18:u8=170
const c23:u8=0
const c28:u8=0
const c42:u8=0
const c46:u8=0
const c54:u8=16
const c70:u8=178
const c74:u8=10
const c83:u8=38
const c90:u8=38
static RESULT:u8
(= RESULT (- (- (- (- (+ 85 c18) c23) c28) 0) (+ (+ c42 c46) (- c54 (- (+ 41 (- c70 c74)) (+ c83 (+ c90 117)))))))
"#);
}
#[test]
fn const_expr_120() {
    test_const_expr(r#"
const c3:u8=36
static RESULT:u8
(= RESULT (+ c3 219))
"#);
}
#[test]
fn const_expr_121() {
    test_const_expr(r#"
const c18:u8=110
const c29:u8=135
const c33:u8=70
const c39:u8=90
const c47:u8=24
const c63:u8=64
const c76:u8=43
const c80:u8=43
static RESULT:u8
(= RESULT (+ (+ (- (+ (- (- c18 28) (- c29 c33)) c39) (+ c47 73)) 53) (+ c63 (+ 42 (+ c76 c80)))))
"#);
}
#[test]
fn const_expr_122() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 85 (+ 85 85)))
"#);
}
#[test]
fn const_expr_123() {
    test_const_expr(r#"
const c6:u8=63
const c9:u8=192
const c31:u8=92
const c42:u8=0
static RESULT:u8
(= RESULT (- (+ c6 c9) (+ (- (+ 22 (+ 22 c31)) 136) c42)))
"#);
}
#[test]
fn const_expr_124() {
    test_const_expr(r#"
const c7:u8=128
static RESULT:u8
(= RESULT (+ 127 c7))
"#);
}
#[test]
fn const_expr_125() {
    test_const_expr(r#"
const c37:u8=6
const c44:u8=60
const c66:u8=231
const c75:u8=9
const c80:u8=0
const c101:u8=19
const c138:u8=251
const c159:u8=12
const c192:u8=83
const c197:u8=79
const c206:u8=43
const c226:u8=5
const c238:u8=180
const c243:u8=0
const c250:u8=2
const c263:u8=210
const c289:u8=29
const c294:u8=90
const c300:u8=119
const c306:u8=0
const c325:u8=211
const c334:u8=185
const c341:u8=0
const c353:u8=13
const c374:u8=0
const c379:u8=0
const c400:u8=39
const c405:u8=119
const c414:u8=0
const c437:u8=165
const c454:u8=222
const c479:u8=229
const c488:u8=8
const c493:u8=43
const c505:u8=203
const c542:u8=93
const c555:u8=164
const c572:u8=0
const c577:u8=6
const c594:u8=134
const c600:u8=103
const c611:u8=0
const c631:u8=29
const c641:u8=59
const c647:u8=80
static RESULT:u8
(= RESULT (- (- (- (- (- (- (+ (+ (- 34 17) (+ c37 (- c44 (- (+ (+ (+ (+ (- c66 227) c75) c80) 27) 205) (- 232 c101))))) (+ (+ (+ 2 6) 32) (- (- (- c138 6) 11) (- (+ (+ c159 12) 50) (+ (- (+ (+ 8 (+ (- c192 c197) (- c206 20))) (- (+ (+ c226 31) (+ c238 c243)) c250)) (+ 35 c263)) (+ 0 (+ 0 (+ (- (+ c289 c294) c300) c306)))))))) (- (- c325 26) c334)) c341) (- 13 c353)) (+ 0 (+ (+ (+ c374 c379) (+ 0 (- 158 (+ c400 c405)))) c414))) (- (- (+ 23 (- c437 (- (+ (- (- c454 88) (+ 33 (+ (- 246 c479) (+ c488 c493)))) (- c505 (- 136 134))) (+ 63 (- (+ 46 (+ c542 93)) (- c555 58)))))) (+ c572 c577)) (+ (- 168 c594) c600))) (+ c611 (- 38 (- (- (+ c631 148) c641) c647)))))
"#);
}
#[test]
fn const_expr_126() {
    test_const_expr(r#"
const c9:u8=39
const c15:u8=39
const c28:u8=250
const c49:u8=222
const c56:u8=20
const c66:u8=69
static RESULT:u8
(= RESULT (+ (- (+ c9 (+ c15 158)) (- c28 65)) (+ (- (- (- c49 (+ c56 (+ 34 c66))) 21) 38) (+ 82 82))))
"#);
}
#[test]
fn const_expr_127() {
    test_const_expr(r#"
const c3:u8=127
static RESULT:u8
(= RESULT (+ c3 128))
"#);
}
#[test]
fn const_expr_128() {
    test_const_expr(r#"
const c7:u8=128
static RESULT:u8
(= RESULT (+ 127 c7))
"#);
}
#[test]
fn const_expr_129() {
    test_const_expr(r#"
const c12:u8=85
const c22:u8=114
const c28:u8=0
const c36:u8=0
const c43:u8=0
const c52:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ c12 (+ 56 c22)) c28) (+ c36 (+ c43 0))) c52))
"#);
}
#[test]
fn const_expr_130() {
    test_const_expr(r#"
const c9:u8=146
const c48:u8=105
const c55:u8=125
const c69:u8=57
const c112:u8=252
const c117:u8=58
const c126:u8=198
const c131:u8=56
const c148:u8=41
const c161:u8=226
const c171:u8=2
const c183:u8=32
const c207:u8=194
const c220:u8=3
const c231:u8=249
const c250:u8=0
const c266:u8=179
const c310:u8=253
const c315:u8=17
const c324:u8=217
const c338:u8=5
const c357:u8=254
const c381:u8=110
const c387:u8=0
const c405:u8=8
const c420:u8=180
const c426:u8=76
const c440:u8=32
const c458:u8=0
static RESULT:u8
(= RESULT (- (+ (- c9 (- (- (- (+ (+ (+ (- (+ (- (- (+ (- c48 (- c55 52)) 132) c69) 103) (+ (+ (- (- (- 218 95) 69) (- (- c112 c117) (- c126 c131))) (- (- (+ c148 168) (- c161 24)) c171)) 23)) c183) (+ 2 (+ (- 128 (- c207 66)) (+ c220 (- (- c231 27) (- (- 254 c250) (+ (- 187 c266) (- 208 184)))))))) 40) (+ (- (- (- (- c310 c315) (- c324 190)) (+ c338 10)) (- (- (- c357 0) (+ (- (+ 27 83) c381) c387)) 87)) 165)) c405) (- (+ 44 c420) c426)) 0)) (+ c440 (+ 32 128))) c458))
"#);
}
#[test]
fn const_expr_131() {
    test_const_expr(r#"
const c6:u8=51
const c12:u8=220
const c21:u8=0
static RESULT:u8
(= RESULT (- (+ c6 (- c12 16)) c21))
"#);
}
#[test]
fn const_expr_132() {
    test_const_expr(r#"
const c15:u8=249
const c52:u8=249
const c66:u8=216
const c81:u8=221
const c94:u8=7
const c105:u8=1
const c128:u8=170
const c143:u8=98
const c151:u8=31
const c156:u8=32
const c181:u8=157
const c201:u8=215
const c224:u8=2
const c235:u8=111
const c244:u8=229
const c255:u8=248
const c262:u8=0
const c278:u8=188
static RESULT:u8
(= RESULT (+ (- (+ (+ (- c15 (+ 164 (- (- 107 (- 111 (+ (- (- c52 (+ (- 216 c66) (+ 0 1))) c81) (+ (+ 2 c94) (+ (+ c105 10) 35))))) 78))) c128) (- (+ (- c143 (+ c151 c156)) (+ (- (- (- (+ (- c181 123) (- 231 (- c201 188))) (+ (+ 0 1) c224)) 53) c235) (- c244 87))) c255)) c262) (- 164 (- c278 24))))
"#);
}
#[test]
fn const_expr_133() {
    test_const_expr(r#"
const c6:u8=51
static RESULT:u8
(= RESULT (- (+ c6 204) 0))
"#);
}
#[test]
fn const_expr_134() {
    test_const_expr(r#"
const c22:u8=104
const c38:u8=51
const c57:u8=117
const c62:u8=179
const c82:u8=115
const c92:u8=8
const c112:u8=246
const c148:u8=88
const c156:u8=219
const c180:u8=0
const c194:u8=234
const c199:u8=158
static RESULT:u8
(= RESULT (- (+ (- (- (+ (- 183 c22) 160) (- 89 c38)) (+ (- (+ 117 c57) c62) (+ (+ (- 36 (- c82 (+ (+ c92 (+ (+ (- 165 (- c112 (- 109 27))) 3) 29)) 42))) 18) c148))) c156) (+ 0 (+ 0 (+ 0 (+ c180 (- 76 (- c194 c199))))))))
"#);
}
#[test]
fn const_expr_135() {
    test_const_expr(r#"
const c27:u8=12
const c37:u8=62
const c52:u8=27
const c71:u8=0
const c81:u8=0
const c99:u8=33
const c103:u8=165
const c118:u8=197
const c131:u8=111
const c151:u8=0
const c162:u8=80
const c175:u8=10
const c203:u8=18
const c218:u8=20
const c228:u8=18
const c290:u8=105
const c299:u8=203
const c314:u8=18
const c331:u8=91
const c347:u8=133
const c354:u8=88
const c365:u8=54
const c391:u8=17
const c416:u8=1
const c421:u8=4
const c428:u8=121
const c435:u8=103
const c456:u8=0
const c470:u8=178
const c475:u8=178
static RESULT:u8
(= RESULT (- (- (- (- (- (+ (+ (- (+ c27 (+ 12 c37)) (- (- (+ c52 (+ (+ (+ (+ (+ c71 0) (+ c81 2)) (- 202 (+ c99 c103))) (- 218 c118)) 140)) c131) (+ 18 (+ 0 (+ c151 (+ (- c162 (+ (+ 3 c175) (- 150 83))) (- (+ (+ c203 0) (- 113 c218)) (+ c228 93)))))))) (- 129 (- (+ (- 97 (- 71 (- 120 (- (+ 67 134) c290)))) c299) (+ (+ (+ c314 (- (- (+ (- c331 56) (- 205 c347)) c354) 19)) c365) 73)))) (- 213 (- (+ c391 (- (- (- 232 20) (+ c416 c421)) c428)) c435))) (- 231 231)) c456) (+ 0 (- c470 c475))) 0) 0))
"#);
}
#[test]
fn const_expr_136() {
    test_const_expr(r#"
const c9:u8=63
const c24:u8=168
const c31:u8=26
const c73:u8=128
const c80:u8=125
const c89:u8=92
const c101:u8=50
const c127:u8=166
const c133:u8=5
const c139:u8=21
const c147:u8=178
const c156:u8=129
const c189:u8=0
const c197:u8=1
const c208:u8=0
const c219:u8=75
const c240:u8=131
const c269:u8=22
const c281:u8=2
const c290:u8=54
const c298:u8=12
const c305:u8=0
const c331:u8=174
const c341:u8=39
const c353:u8=191
const c377:u8=31
const c382:u8=9
const c406:u8=58
const c430:u8=139
const c446:u8=143
const c453:u8=141
const c481:u8=47
const c489:u8=15
const c501:u8=103
const c521:u8=0
static RESULT:u8
(= RESULT (- (- (+ c9 (- (+ (- (- c24 (+ c31 (- (+ 32 (+ (+ (- 136 129) (+ 3 (- (- c73 (- c80 93)) c89))) 84)) c101))) (+ 5 (+ (- (- 178 c127) c133) c139))) c147) (- c156 (+ (- (- (+ (- 236 (+ (+ (+ c189 (+ c197 (+ (+ c208 0) (- c219 74)))) (- (- (+ c240 (- 68 68)) 64) (+ (+ (- c269 17) (+ c281 3)) c290))) c298)) c305) 70) (- (- (+ 109 (- c331 64)) c341) (- (+ c353 (- (- 236 (- (+ (- c377 c382) 133) (+ (- (- 243 c406) (- 241 71)) (- (- c430 76) 16)))) c446)) c453))) (+ 15 80))))) (- (+ c481 (+ c489 (- 135 c501))) (+ 15 79))) c521))
"#);
}
#[test]
fn const_expr_137() {
    test_const_expr(r#"
const c10:u8=20
static RESULT:u8
(= RESULT (+ (- 147 c10) 128))
"#);
}
#[test]
fn const_expr_138() {
    test_const_expr(r#"
const c46:u8=230
const c60:u8=135
const c64:u8=58
const c73:u8=24
const c80:u8=36
const c87:u8=249
const c94:u8=187
const c98:u8=49
const c124:u8=6
const c129:u8=34
const c141:u8=50
const c160:u8=111
const c179:u8=174
const c193:u8=250
const c211:u8=200
const c219:u8=26
const c235:u8=0
const c246:u8=0
const c257:u8=0
const c270:u8=116
const c285:u8=5
const c357:u8=8
const c367:u8=150
const c384:u8=6
const c399:u8=229
const c416:u8=127
const c435:u8=70
const c447:u8=176
const c461:u8=245
const c474:u8=62
const c518:u8=244
const c523:u8=1
const c572:u8=2
const c581:u8=116
const c601:u8=169
const c609:u8=24
const c621:u8=0
const c645:u8=32
const c667:u8=169
const c673:u8=23
static RESULT:u8
(= RESULT (+ (+ 63 (- (+ 5 (- 238 (+ (- (- 238 (+ 17 (- c46 142))) (- c60 c64)) (+ c73 (+ c80 (- c87 (- c94 c98))))))) (+ (- (+ (+ (+ c124 c129) (- (+ c141 152) 202)) (- c160 (- (- 229 25) c179))) (- (- c193 (+ (- 178 (- c211 (+ c219 (+ 0 (+ (+ c235 (+ (+ c246 0) (+ c257 0))) (- c270 116)))))) c285)) (+ 25 (+ 14 (- (- (- 234 19) 116) (+ (- (+ 39 (+ (+ (- 23 15) (+ c357 33)) c367)) 232) (+ 1 c384))))))) (- c399 (+ (+ 10 (- c416 (- 217 (- 193 c435)))) (- c447 (- (- (+ c461 0) 182) c474))))))) (+ (- (- 199 32) (- (- (+ (- (- c518 c523) 161) (- (- (- (+ 63 189) 7) 67) (+ (+ (+ 0 c572) (- c581 116)) 12))) (- c601 (+ c609 (+ 124 c621)))) 154)) (+ (+ 16 c645) (- 236 (- (+ 42 c667) c673))))))
"#);
}
#[test]
fn const_expr_139() {
    test_const_expr(r#"
const c12:u8=106
const c23:u8=62
const c33:u8=93
const c45:u8=111
const c66:u8=0
const c85:u8=63
const c91:u8=56
static RESULT:u8
(= RESULT (- (+ (+ 21 c12) (- (+ c23 (+ 31 c33)) (+ (- c45 53) 0))) (+ 0 (+ c66 (- (- 76 (- 83 c85)) c91)))))
"#);
}
#[test]
fn const_expr_140() {
    test_const_expr(r#"
const c3:u8=42
const c6:u8=213
static RESULT:u8
(= RESULT (+ c3 c6))
"#);
}
#[test]
fn const_expr_141() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 127 128))
"#);
}
#[test]
fn const_expr_142() {
    test_const_expr(r#"
const c52:u8=168
const c57:u8=61
const c62:u8=18
const c67:u8=108
const c76:u8=102
const c101:u8=28
const c123:u8=107
const c128:u8=77
const c161:u8=13
const c174:u8=27
const c187:u8=146
const c192:u8=134
const c234:u8=125
const c239:u8=125
const c248:u8=4
const c259:u8=31
const c270:u8=167
const c280:u8=69
const c285:u8=0
const c308:u8=48
const c313:u8=192
const c319:u8=2
const c328:u8=228
const c333:u8=224
const c343:u8=127
const c365:u8=0
const c374:u8=0
const c382:u8=0
const c387:u8=0
const c401:u8=0
const c406:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (+ (- (+ 12 (- (- 215 (+ (- (- (- (+ 56 c52) c57) c62) c67) 0)) c76)) (- 147 144)) (- (+ c101 144) (- (- (+ (- c123 c128) (+ (- (+ 23 (- (- (+ (+ 13 c161) (- 161 c174)) (+ (- c187 c192) 13)) (+ 44 (+ 15 30)))) (- (- (- (+ c234 c239) (+ c248 (- 56 c259))) 7) c270)) (+ c280 c285))) (- (- (- (- (+ c308 c313) c319) (- c328 c333)) (- c343 77)) 137)) 72))) c365) (+ c374 (+ c382 c387))) 0) (+ c401 c406)))
"#);
}
#[test]
fn const_expr_143() {
    test_const_expr(r#"
const c24:u8=36
const c51:u8=26
const c75:u8=102
const c85:u8=0
const c89:u8=2
const c97:u8=236
static RESULT:u8
(= RESULT (- (- (- (+ 36 (+ 36 (+ c24 (+ 24 123)))) 0) (+ (- c51 26) 0)) (+ (- 86 (- c75 (+ (+ c85 c89) (- c97 222)))) 0)))
"#);
}
#[test]
fn const_expr_144() {
    test_const_expr(r#"
const c6:u8=9
const c15:u8=14
const c21:u8=219
static RESULT:u8
(= RESULT (+ (+ c6 (+ 13 c15)) c21))
"#);
}
#[test]
fn const_expr_145() {
    test_const_expr(r#"
const c27:u8=254
const c41:u8=5
const c46:u8=121
const c87:u8=36
const c102:u8=0
static RESULT:u8
(= RESULT (- (+ 51 (+ (+ 8 26) (- (- c27 (- (- 186 c41) c46)) 24))) (+ (+ (- (- 149 7) (- 250 (+ c87 72))) (+ 0 c102)) 0)))
"#);
}
#[test]
fn const_expr_146() {
    test_const_expr(r#"
const c12:u8=204
const c35:u8=0
const c44:u8=59
const c62:u8=0
const c75:u8=28
const c79:u8=145
const c93:u8=3
const c102:u8=11
const c119:u8=0
const c134:u8=6
const c146:u8=60
const c151:u8=121
const c157:u8=5
const c165:u8=4
static RESULT:u8
(= RESULT (- (- (+ 51 c12) 0) (- (- 99 (+ 66 c35)) (- c44 (+ (+ 0 (+ (+ c62 (+ (- (+ c75 c79) 173) 0)) c93)) (+ c102 (- 16 (+ (+ c119 (- (- 182 c134) (- (+ c146 c151) c157))) c165))))))))
"#);
}
#[test]
fn const_expr_147() {
    test_const_expr(r#"
const c9:u8=42
const c26:u8=40
const c33:u8=102
const c72:u8=114
const c76:u8=114
const c105:u8=243
const c122:u8=56
const c137:u8=0
const c142:u8=0
const c163:u8=26
const c168:u8=160
const c174:u8=183
const c184:u8=33
const c194:u8=185
const c203:u8=47
const c215:u8=2
const c230:u8=109
const c238:u8=109
const c256:u8=43
const c280:u8=63
const c294:u8=143
static RESULT:u8
(= RESULT (- (- (+ c9 213) (- (- (+ c26 (+ c33 (- (+ (- 63 (- 136 (- (- (- 254 (- c72 c76)) 13) (- (+ 156 (+ (- (- c105 112) (- 187 c122)) (+ 0 (+ c137 c142)))) (+ (+ (- (+ c163 c168) c174) 13) c184))))) c194) (+ c203 96)))) c215) 243)) (- c230 (+ c238 (- 119 (- (+ c256 (- (- 217 (+ 15 (- c280 17))) (- c294 119))) (+ (- 52 24) 28)))))))
"#);
}
#[test]
fn const_expr_148() {
    test_const_expr(r#"
const c21:u8=29
const c26:u8=35
const c32:u8=204
static RESULT:u8
(= RESULT (- (+ (- 120 (+ (+ 5 c21) c26)) c32) 0))
"#);
}
#[test]
fn const_expr_149() {
    test_const_expr(r#"
const c9:u8=25
const c32:u8=189
const c39:u8=20
const c78:u8=0
const c114:u8=0
const c119:u8=2
const c135:u8=62
const c151:u8=233
const c159:u8=157
const c167:u8=0
const c172:u8=0
const c210:u8=10
const c217:u8=26
const c223:u8=31
const c229:u8=114
const c236:u8=213
static RESULT:u8
(= RESULT (+ (+ (- c9 (- (- 219 79) (+ (- c32 (+ c39 40)) 0))) (- (- (- (- 212 (+ (+ (+ c78 (- 73 (- 135 (+ (- 147 (+ (+ (+ c114 c119) (+ 18 0)) c135)) (- 76 (- c151 (+ c159 (+ c167 c172)))))))) (+ (+ 0 (- 129 129)) 0)) c210)) c217) c223) c229)) c236))
"#);
}
#[test]
fn const_expr_150() {
    test_const_expr(r#"
const c32:u8=42
const c41:u8=79
const c65:u8=75
const c69:u8=75
const c80:u8=51
const c84:u8=103
const c101:u8=254
const c109:u8=246
const c120:u8=19
const c131:u8=154
const c165:u8=28
const c180:u8=228
const c185:u8=53
const c203:u8=10
const c215:u8=3
const c220:u8=18
const c248:u8=8
const c261:u8=38
const c286:u8=110
const c296:u8=192
static RESULT:u8
(= RESULT (+ (- 178 (+ (- (+ (- 160 (- (+ c32 169) c41)) (+ (- 29 (+ (+ (- c65 c69) (- (+ c80 c84) 154)) (- (- c101 7) c109))) (+ c120 95))) c131) (+ 16 (- (+ (+ (- (+ 16 32) c165) (+ (- (- c180 c185) (- (+ (- (+ c203 53) (+ c215 c220)) (+ (- 158 137) 108)) c248)) (+ 12 c261))) 165) (+ (+ 7 47) c286))))) c296))
"#);
}
#[test]
fn const_expr_151() {
    test_const_expr(r#"
const c24:u8=244
const c52:u8=0
static RESULT:u8
(= RESULT (- (+ (- (+ (+ 15 (- (- c24 229) 0)) 180) 168) 213) c52))
"#);
}
#[test]
fn const_expr_152() {
    test_const_expr(r#"
const c42:u8=229
const c52:u8=0
const c63:u8=217
const c88:u8=252
const c104:u8=8
const c140:u8=233
const c185:u8=106
const c207:u8=82
const c219:u8=176
const c257:u8=44
const c266:u8=175
const c304:u8=17
const c309:u8=104
const c325:u8=155
const c336:u8=22
const c345:u8=64
const c350:u8=130
const c357:u8=9
static RESULT:u8
(= RESULT (- (- (- (- (- (+ 51 (+ (+ 34 0) 170)) (- c42 229)) c52) (- (- c63 0) (- (+ (+ (- (- (- c88 13) 195) (+ c104 (- (- 106 (+ (- 183 153) (+ (- c140 (+ 77 156)) (- (- (+ 54 109) (+ 11 46)) c185)))) (- (+ (- 111 c207) (- (+ c219 (- (- 118 12) 106)) (- (- (- 246 c257) 0) c266))) (- (- 254 120) (- (+ 40 (+ (+ c304 c309) (+ 0 0))) c325)))))) c336) (+ c345 c350)) c357))) 0) 0))
"#);
}
#[test]
fn const_expr_153() {
    test_const_expr(r#"
const c7:u8=128
static RESULT:u8
(= RESULT (+ 127 c7))
"#);
}
#[test]
fn const_expr_154() {
    test_const_expr(r#"
const c18:u8=117
const c25:u8=216
const c43:u8=0
const c59:u8=0
const c85:u8=0
const c91:u8=12
static RESULT:u8
(= RESULT (- (- (- (+ (+ (- c18 (- c25 184)) 0) 170) c43) 0) (+ (+ 0 c59) (- 14 (+ (- 61 (+ 59 c85)) c91)))))
"#);
}
#[test]
fn const_expr_155() {
    test_const_expr(r#"
const c29:u8=238
const c53:u8=0
const c61:u8=139
const c73:u8=137
const c108:u8=146
const c114:u8=84
const c123:u8=96
const c140:u8=37
const c163:u8=240
const c168:u8=20
const c174:u8=128
const c193:u8=74
const c205:u8=96
const c234:u8=2
const c244:u8=79
const c250:u8=48
const c304:u8=32
const c313:u8=32
const c318:u8=160
const c330:u8=30
const c353:u8=2
const c374:u8=29
const c382:u8=62
const c438:u8=199
const c475:u8=217
const c480:u8=88
const c502:u8=11
const c514:u8=0
const c523:u8=0
const c573:u8=11
const c606:u8=0
const c611:u8=0
const c648:u8=27
const c667:u8=0
const c693:u8=130
const c704:u8=0
const c709:u8=0
const c718:u8=0
const c728:u8=116
const c739:u8=107
const c754:u8=28
const c765:u8=30
const c781:u8=132
const c789:u8=132
const c811:u8=39
const c816:u8=80
const c832:u8=163
const c840:u8=11
const c848:u8=11
const c853:u8=58
const c870:u8=72
const c889:u8=1
const c905:u8=248
const c916:u8=46
const c924:u8=46
const c946:u8=4
const c956:u8=120
const c972:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ 9 (- (+ (+ (- (- c29 (- 193 (- 121 (- 15 c53)))) c61) (- 175 c73)) (- (+ (+ (+ (- (- (+ 32 198) c108) c114) (- c123 (- (- (- (+ c140 (- 205 20)) (- (- c163 c168) c174)) (+ (- (+ 37 c193) (+ 15 c205)) 5)) (- 159 127)))) (+ c234 14)) c244) c250)) (+ (- (- 212 (- (+ (+ (+ (- (- 253 198) 40) 0) c304) (+ c313 c318)) 66)) c330) (+ (- (+ (+ 6 (+ c353 (+ (- (- 198 (- c374 (- c382 34))) (- 236 44)) (- (- 189 13) 165)))) 150) (- (- c438 (- 200 195)) 32)) (+ (+ 4 (- (- c475 c480) (+ (+ (- 35 25) c502) (+ 87 c514)))) c523))))) (- (- 224 (- (- (- 91 (+ (- (+ (- 39 (+ c573 (- 20 9))) (+ (+ (+ (+ 0 (+ c606 c611)) (- (- 241 6) 234)) (- (+ 5 (+ c648 0)) (- (- 254 c667) (+ 32 192)))) (- (+ c693 (+ (+ c704 c709) (+ c718 0))) c728))) (- c739 92)) 19)) c754) 23)) c765)) (- (+ (- c781 (- c789 (- 133 (- 197 (+ c811 c816))))) (+ (- c832 (+ c840 (+ c848 c853))) (- (+ 24 c870) (+ 2 (+ (+ 0 c889) 9))))) (- c905 (- (+ c916 (+ c924 (+ (+ (+ 0 2) (+ c946 14)) c956))) 206)))) c972))
"#);
}
#[test]
fn const_expr_156() {
    test_const_expr(r#"
const c18:u8=149
const c25:u8=18
const c29:u8=75
const c41:u8=203
const c45:u8=17
const c77:u8=0
const c85:u8=33
const c90:u8=43
const c100:u8=48
const c115:u8=164
static RESULT:u8
(= RESULT (+ (- (+ 85 (+ (- c18 (+ c25 c29)) (- (- c41 c45) (+ (- (- 42 (- (+ (+ 11 (+ c77 0)) c85) c90)) 17) c100)))) 0) (- c115 164)))
"#);
}
#[test]
fn const_expr_157() {
    test_const_expr(r#"
const c9:u8=192
const c13:u8=0
static RESULT:u8
(= RESULT (- (+ 63 c9) c13))
"#);
}
#[test]
fn const_expr_158() {
    test_const_expr(r#"
const c6:u8=51
const c17:u8=158
const c29:u8=19
const c41:u8=11
static RESULT:u8
(= RESULT (- (+ c6 204) (- c17 (+ (+ 3 c29) (- 147 c41)))))
"#);
}
#[test]
fn const_expr_159() {
    test_const_expr(r#"
const c18:u8=206
const c27:u8=0
const c38:u8=29
const c51:u8=140
const c61:u8=0
const c92:u8=34
const c109:u8=18
const c126:u8=31
const c139:u8=27
const c147:u8=33
const c165:u8=134
const c171:u8=142
const c179:u8=17
static RESULT:u8
(= RESULT (- (+ (- (+ (+ (- c18 155) c27) (+ (+ c38 0) (+ 35 c51))) (+ c61 0)) (- (- (- 251 36) 9) (+ c92 (- (- 225 (+ c109 (- (+ (- (+ c126 189) (+ c139 (+ c147 (+ 44 90)))) c165) c171))) c179)))) 0))
"#);
}
#[test]
fn const_expr_160() {
    test_const_expr(r#"
const c26:u8=90
const c40:u8=9
const c60:u8=24
const c81:u8=8
const c88:u8=1
const c98:u8=222
static RESULT:u8
(= RESULT (- (+ 42 (+ 213 0)) (- (+ c26 90) (+ (+ c40 (- 195 (+ 39 (+ c60 96)))) (+ 135 (- c81 (+ c88 (+ (- c98 221) 6))))))))
"#);
}
#[test]
fn const_expr_161() {
    test_const_expr(r#"
const c15:u8=51
const c25:u8=4
const c29:u8=30
const c52:u8=15
const c93:u8=10
const c98:u8=80
const c113:u8=167
const c126:u8=113
const c157:u8=2
const c162:u8=8
const c168:u8=43
const c183:u8=130
const c195:u8=22
const c209:u8=23
const c220:u8=113
const c233:u8=0
const c238:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (+ c15 (+ (+ c25 c29) 170)) (+ (- 90 (+ c52 (+ 18 (+ (- (- (- (- 224 (- (+ (+ 10 c93) c98) (+ (- 197 c113) (- 175 c126)))) (+ (- (+ 13 78) (+ (+ c157 c162) c168)) 0)) 37) c183) (- (+ c195 (+ 45 (+ c209 69))) c220))))) (+ c233 c238))) (- (- (- 222 (- 205 73)) 59) 31)) 0) (+ 0 0)))
"#);
}
#[test]
fn const_expr_162() {
    test_const_expr(r#"
const c33:u8=4
const c37:u8=26
const c48:u8=11
const c59:u8=11
const c69:u8=56
const c90:u8=34
const c101:u8=4
const c111:u8=219
const c129:u8=206
const c135:u8=0
const c144:u8=10
const c156:u8=194
const c177:u8=6
const c188:u8=0
const c194:u8=128
static RESULT:u8
(= RESULT (+ (+ (+ (+ 4 (+ (+ (- (- 231 (+ c33 c37)) 199) c48) (- (+ c59 (+ 13 c69)) (+ (+ (- (- (+ c90 (- 211 c101)) (- c111 (- 228 31))) c129) c135) (+ c144 (- 237 c156)))))) (- 140 (+ c177 38))) c188) c194))
"#);
}
#[test]
fn const_expr_163() {
    test_const_expr(r#"
const c35:u8=38
const c79:u8=55
const c90:u8=117
const c96:u8=19
const c101:u8=0
const c109:u8=51
const c119:u8=216
static RESULT:u8
(= RESULT (+ (+ (- (+ 30 31) (- (- 241 (- (+ c35 195) 57)) 21)) (+ (- 124 (+ 15 (+ (- (+ c79 (- 173 c90)) c96) c101))) c109)) (- c119 46)))
"#);
}
#[test]
fn const_expr_164() {
    test_const_expr(r#"
const c30:u8=63
const c34:u8=190
const c45:u8=122
const c61:u8=243
const c68:u8=3
const c79:u8=98
const c92:u8=0
const c103:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- (+ 63 (+ (- (- (+ c30 c34) (+ 60 c45)) 44) (- (- c61 (+ c68 (- 117 c79))) 56))) c92) 0) 0) c103))
"#);
}
#[test]
fn const_expr_165() {
    test_const_expr(r#"
const c6:u8=192
static RESULT:u8
(= RESULT (+ 63 c6))
"#);
}
#[test]
fn const_expr_166() {
    test_const_expr(r#"
const c24:u8=252
const c28:u8=0
const c33:u8=189
const c43:u8=0
const c100:u8=5
const c118:u8=119
const c130:u8=56
const c148:u8=140
const c168:u8=61
const c187:u8=252
const c192:u8=9
const c218:u8=160
const c241:u8=115
const c249:u8=72
const c262:u8=44
const c281:u8=0
const c289:u8=0
const c295:u8=0
const c304:u8=21
const c309:u8=21
const c317:u8=84
const c338:u8=159
const c346:u8=39
const c370:u8=61
const c405:u8=106
const c411:u8=34
const c417:u8=94
const c435:u8=220
const c443:u8=234
const c454:u8=202
const c465:u8=225
const c470:u8=219
const c476:u8=34
const c489:u8=23
const c497:u8=5
const c516:u8=109
const c529:u8=156
const c541:u8=26
const c564:u8=245
const c569:u8=35
const c596:u8=33
const c613:u8=86
const c624:u8=5
const c632:u8=146
const c639:u8=73
static RESULT:u8
(= RESULT (- (- (- (- (- (+ (- (- c24 c28) c33) 192) c43) 0) 0) 0) (- (- (+ (- (+ (- 86 40) (- (+ (+ (- (+ (+ c100 (- (+ 165 (- c118 (- 175 c130))) (+ (- 210 c148) (- 250 (- 240 c168))))) (- (- (- c187 c192) 17) (+ (- (- (- 213 c218) (- 228 227)) 29) c241))) c249) (- 224 c262)) (+ (+ (+ (+ c281 0) c289) c295) (- c304 c309))) c317)) (+ 10 21)) (- c338 (+ c346 120))) (- (- (- (+ c370 (- (+ 37 (+ (- (+ (- 184 163) c405) c411) c417)) (+ (+ 4 (- c435 (- c443 (- (- c454 (+ (- c465 c470) c476)) (+ (+ c489 (+ c497 18)) (- (+ 18 c516) (- 189 c529))))))) c541))) (- 73 (- (- (- c564 c569) 101) (- (- 145 (+ 33 c596)) 31)))) (- c613 (- 85 c624))) c632)) c639)))
"#);
}
#[test]
fn const_expr_167() {
    test_const_expr(r#"
const c25:u8=149
const c33:u8=124
const c44:u8=1
const c51:u8=4
static RESULT:u8
(= RESULT (- (+ 127 (+ (- (- (+ (- c25 26) c33) (+ (+ c44 (+ c51 0)) 30)) 170) (- 202 116))) 0))
"#);
}
#[test]
fn const_expr_168() {
    test_const_expr(r#"
const c15:u8=213
const c20:u8=0
const c28:u8=78
const c35:u8=13
const c70:u8=130
const c79:u8=93
const c87:u8=3
const c94:u8=145
const c118:u8=243
const c126:u8=253
static RESULT:u8
(= RESULT (- (- (- (+ 42 c15) c20) (- c28 (+ c35 65))) (- (- (- (- (+ (- (- 232 c70) 56) c79) (+ c87 (- c94 126))) 12) (+ 97 (- c118 (- c126 10)))) 8)))
"#);
}
#[test]
fn const_expr_169() {
    test_const_expr(r#"
const c9:u8=225
const c18:u8=190
const c34:u8=28
const c38:u8=144
const c43:u8=160
const c66:u8=0
static RESULT:u8
(= RESULT (- (+ (- c9 (- (- c18 (+ 12 (- (+ c34 c38) c43))) 4)) (+ 32 160)) c66))
"#);
}
#[test]
fn const_expr_170() {
    test_const_expr(r#"
const c18:u8=109
const c28:u8=0
const c32:u8=0
const c37:u8=0
const c43:u8=33
const c79:u8=40
const c83:u8=164
static RESULT:u8
(= RESULT (+ (- (+ 75 (- (+ c18 (+ (+ c28 c32) c37)) c43)) (- 199 (- (- 194 33) 62))) (+ c79 c83)))
"#);
}
#[test]
fn const_expr_171() {
    test_const_expr(r#"
const c11:u8=10
const c27:u8=2
const c31:u8=13
const c36:u8=15
const c62:u8=0
const c67:u8=4
const c75:u8=33
const c82:u8=16
const c92:u8=48
const c111:u8=164
const c129:u8=10
const c159:u8=210
const c176:u8=112
const c184:u8=44
const c194:u8=3
static RESULT:u8
(= RESULT (+ (+ (+ 2 c11) (+ 9 (+ (+ c27 c31) c36))) (+ 29 (- (- (+ 245 c62) c67) (+ c75 (+ c82 (+ (- c92 (- (- (+ 69 (- c111 (- (- (- 253 c129) 14) 204))) (+ (+ (- 216 c159) 24) (- 203 c176))) c184)) (+ c194 9))))))))
"#);
}
#[test]
fn const_expr_172() {
    test_const_expr(r#"
const c9:u8=155
const c12:u8=92
const c17:u8=192
const c22:u8=0
static RESULT:u8
(= RESULT (- (+ (- c9 c12) c17) c22))
"#);
}
#[test]
fn const_expr_173() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 63 192))
"#);
}
#[test]
fn const_expr_174() {
    test_const_expr(r#"
const c16:u8=38
const c20:u8=76
const c34:u8=197
const c50:u8=12
const c54:u8=75
const c60:u8=58
static RESULT:u8
(= RESULT (+ (+ (- 135 (+ c16 c20)) 106) (- c34 (+ (- 98 (+ c50 c54)) c60))))
"#);
}
#[test]
fn const_expr_175() {
    test_const_expr(r#"
const c15:u8=34
const c19:u8=13
const c39:u8=86
const c71:u8=231
const c91:u8=173
const c105:u8=12
const c117:u8=16
const c128:u8=86
const c136:u8=0
const c153:u8=32
const c163:u8=244
const c173:u8=0
const c194:u8=1
const c199:u8=7
const c210:u8=70
static RESULT:u8
(= RESULT (- (- (+ (+ (- c15 c19) (+ (+ (- (+ (- c39 (+ (+ 17 0) 0)) (+ 23 (+ (- c71 (- 222 (+ (- (- c91 25) (+ (+ c105 51) (+ c117 48))) c128))) c136))) 203) 27) c153)) (- c163 74)) c173) (- (+ 17 (+ (+ c194 c199) 45)) c210)))
"#);
}
#[test]
fn const_expr_176() {
    test_const_expr(r#"
const c6:u8=116
const c9:u8=80
const c16:u8=219
static RESULT:u8
(= RESULT (+ (- c6 c9) (+ c16 0)))
"#);
}
#[test]
fn const_expr_177() {
    test_const_expr(r#"
const c12:u8=15
const c73:u8=22
const c77:u8=23
const c86:u8=0
const c103:u8=102
const c108:u8=62
const c117:u8=28
const c122:u8=172
const c129:u8=30
const c138:u8=231
const c143:u8=23
const c161:u8=120
const c167:u8=236
const c209:u8=3
const c222:u8=127
const c237:u8=4
const c265:u8=240
const c270:u8=11
const c284:u8=6
const c290:u8=18
const c334:u8=47
const c342:u8=3
const c360:u8=181
const c385:u8=75
const c418:u8=25
const c423:u8=0
const c451:u8=59
const c468:u8=38
const c491:u8=23
const c514:u8=123
const c522:u8=54
const c531:u8=103
const c553:u8=0
static RESULT:u8
(= RESULT (- (- (+ (+ c12 48) (+ 96 96)) (- (+ (- 199 167) (+ (+ (+ (+ (+ (+ (- (+ c73 c77) 45) c86) (- (- (+ (- c103 c108) (+ c117 c122)) c129) (- c138 c143))) (- (+ 119 c161) c167)) (- (+ (- (- (- (- 238 (+ (+ 1 (+ 0 c209)) (- (- c222 81) 19))) c237) 51) (+ (- (+ 50 (- (- c265 c270) (+ (+ 2 c284) c290))) (+ 26 161)) (+ (- 186 164) (- (+ (- c334 (+ c342 10)) 172) (- c360 (- (+ 88 88) (- 231 c385))))))) (- 101 44)) (- (+ (+ c418 c423) (+ (- (- (- 202 (- (+ c451 120) (- 167 c468))) (- (- 250 (+ 4 c491)) 188)) 96) 131)) c514))) c522) (- c531 21))) 195)) (+ 0 c553)))
"#);
}
#[test]
fn const_expr_178() {
    test_const_expr(r#"
const c19:u8=4
static RESULT:u8
(= RESULT (+ 51 (+ 51 (- 157 c19))))
"#);
}
#[test]
fn const_expr_179() {
    test_const_expr(r#"
const c3:u8=63
const c13:u8=17
static RESULT:u8
(= RESULT (+ c3 (- 209 c13)))
"#);
}
#[test]
fn const_expr_180() {
    test_const_expr(r#"
const c9:u8=219
const c13:u8=0
static RESULT:u8
(= RESULT (- (+ 36 c9) c13))
"#);
}
#[test]
fn const_expr_181() {
    test_const_expr(r#"
const c6:u8=85
const c14:u8=0
static RESULT:u8
(= RESULT (- (+ c6 170) c14))
"#);
}
#[test]
fn const_expr_182() {
    test_const_expr(r#"
const c6:u8=51
const c9:u8=204
const c19:u8=0
const c23:u8=0
static RESULT:u8
(= RESULT (- (+ c6 c9) (+ (+ c19 c23) (+ 0 0))))
"#);
}
#[test]
fn const_expr_183() {
    test_const_expr(r#"
const c26:u8=245
const c30:u8=183
const c64:u8=0
const c73:u8=2
const c85:u8=0
const c98:u8=178
const c102:u8=170
const c122:u8=205
const c131:u8=7
const c136:u8=32
const c160:u8=41
const c179:u8=173
const c204:u8=0
const c217:u8=13
const c247:u8=0
static RESULT:u8
(= RESULT (- (+ (- (- 195 28) (- (- c26 c30) (+ (+ (+ (+ 0 0) 1) 2) (+ (+ c64 (+ 1 c73)) (+ (+ c85 (- (+ (- c98 c102) 45) (+ (- 217 c122) (+ c131 c136)))) (- (- (+ (- 75 c160) (+ (+ 11 23) c179)) 181) (- (+ (+ 104 c204) (- 117 c217)) 162))))))) (+ 25 103)) c247))
"#);
}
#[test]
fn const_expr_184() {
    test_const_expr(r#"
const c6:u8=63
const c14:u8=0
static RESULT:u8
(= RESULT (- (+ c6 192) c14))
"#);
}
#[test]
fn const_expr_185() {
    test_const_expr(r#"
const c19:u8=88
const c49:u8=137
const c65:u8=79
const c69:u8=79
const c96:u8=199
const c125:u8=109
const c145:u8=98
const c167:u8=22
const c175:u8=12
const c180:u8=77
const c197:u8=7
const c202:u8=16
const c215:u8=116
const c223:u8=146
const c249:u8=249
const c268:u8=74
const c273:u8=75
const c288:u8=48
const c294:u8=31
const c303:u8=192
const c308:u8=152
const c330:u8=154
const c337:u8=6
const c371:u8=0
const c393:u8=78
const c401:u8=194
const c440:u8=195
const c445:u8=142
const c451:u8=107
const c458:u8=191
const c483:u8=169
const c505:u8=6
const c510:u8=40
const c523:u8=20
const c530:u8=136
static RESULT:u8
(= RESULT (- (- (+ (- 212 (+ c19 88)) 219) (- 108 (+ (+ (- c49 (+ 22 (- (+ c65 c69) (+ (- 220 (+ (- (+ (- c96 (- 193 38)) (- (- 198 (- c125 101)) (- (+ 48 c145) (+ 29 59)))) (+ c167 (+ c175 c180))) 132)) (+ c197 c202))))) (- c215 (- c223 (+ (+ (- (- (+ (- (- c249 185) (- (- (+ c268 c273) 42) 67)) c288) c294) (- c303 c308)) (+ (+ 0 (- 155 c330)) c337)) (- (- (- (- (- 254 1) (+ 0 c371)) (+ 2 (- 161 (+ c393 (- c401 116))))) (- (+ 215 (- (+ 31 (+ (- c440 c445) c451)) c458)) (- (+ (- 147 105) c483) (- 166 (- (+ (+ c505 c510) (- 161 c523)) c530))))) 87))))) 87))) 0))
"#);
}
#[test]
fn const_expr_186() {
    test_const_expr(r#"
const c9:u8=213
const c13:u8=0
static RESULT:u8
(= RESULT (- (+ 42 c9) c13))
"#);
}
#[test]
fn const_expr_187() {
    test_const_expr(r#"
const c15:u8=67
const c27:u8=66
const c51:u8=200
const c60:u8=242
const c79:u8=235
const c83:u8=6
const c109:u8=113
const c124:u8=3
const c132:u8=89
const c143:u8=39
const c186:u8=29
const c207:u8=1
const c231:u8=101
const c236:u8=97
const c242:u8=25
const c262:u8=35
const c276:u8=30
const c285:u8=243
const c322:u8=17
const c338:u8=29
const c344:u8=145
const c351:u8=41
const c370:u8=254
const c405:u8=8
const c423:u8=75
const c433:u8=25
const c443:u8=220
const c461:u8=124
const c471:u8=57
static RESULT:u8
(= RESULT (+ (+ (+ (- (+ c15 134) (+ c27 134)) (- (- 243 32) c51)) (- c60 169)) (- (- (- c79 c83) (- (+ 31 (- (+ (- (- c109 26) (+ (+ c124 (- c132 (- (+ c143 120) 76))) (+ 6 (- 54 12)))) (+ (- (+ c186 58) (- (+ (+ (+ c207 7) 32) (- 70 (+ (- c231 c236) c242))) (- (- (- (+ c262 (+ (+ 29 c276) (- c285 124))) 74) (+ (+ (+ (- 172 169) c322) (- (- 195 c338) c344)) c351)) 32))) (- (- c370 87) (+ (- 174 (+ 162 (- 58 (+ c405 50)))) (- 75 c423))))) c433)) (+ c443 (- 185 (+ 61 c461))))) c471)))
"#);
}
#[test]
fn const_expr_188() {
    test_const_expr(r#"
const c3:u8=36
const c18:u8=73
static RESULT:u8
(= RESULT (+ c3 (+ 73 (+ 73 c18))))
"#);
}
#[test]
fn const_expr_189() {
    test_const_expr(r#"
const c12:u8=4
const c58:u8=151
const c72:u8=0
const c78:u8=152
const c96:u8=185
const c106:u8=35
const c123:u8=20
const c155:u8=31
const c160:u8=158
const c180:u8=22
const c186:u8=88
const c194:u8=13
const c206:u8=169
const c221:u8=165
const c229:u8=133
const c234:u8=0
static RESULT:u8
(= RESULT (+ (- 87 (+ c12 (- (- (+ (- (- (+ (+ (+ 62 (+ (- 237 200) c58)) 0) (+ 0 c72)) c78) (+ (- 189 (- c96 (- (+ c106 179) (- 222 c123)))) (- 227 177))) (+ (- (+ c155 c160) 123) (+ (+ 22 c180) c186))) c194) (+ 28 c206)))) (+ (- c221 (+ c229 c234)) 160)))
"#);
}
#[test]
fn const_expr_190() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (- (+ 51 204) 0))
"#);
}
#[test]
fn const_expr_191() {
    test_const_expr(r#"
const c21:u8=235
const c35:u8=174
const c39:u8=1
const c46:u8=0
const c51:u8=183
const c56:u8=73
const c61:u8=36
const c84:u8=2
const c100:u8=188
const c125:u8=31
const c130:u8=0
const c136:u8=190
const c151:u8=218
const c156:u8=122
const c162:u8=76
const c174:u8=238
const c179:u8=13
const c192:u8=102
const c212:u8=69
const c218:u8=34
const c230:u8=164
const c237:u8=0
static RESULT:u8
(= RESULT (+ (+ (- (+ (- (+ (- c21 (- 211 (- c35 c39))) c46) c51) c56) c61) (+ (+ 13 (+ (+ (+ c84 4) (- (- (- c100 (+ (+ 8 (- (- (+ (+ c125 c130) c136) (+ (- (- c151 c156) c162) (- (- c174 c179) (+ 102 c192)))) 162)) 52)) c212) c218)) 14)) c230)) c237))
"#);
}
#[test]
fn const_expr_192() {
    test_const_expr(r#"
const c37:u8=195
const c41:u8=167
const c46:u8=58
const c58:u8=0
const c85:u8=80
const c93:u8=139
const c114:u8=40
const c120:u8=192
const c126:u8=69
const c144:u8=15
const c149:u8=15
const c166:u8=220
const c177:u8=1
const c204:u8=105
const c225:u8=35
const c230:u8=144
const c236:u8=105
const c265:u8=82
const c279:u8=95
const c294:u8=108
const c347:u8=13
const c371:u8=113
const c383:u8=34
const c407:u8=250
const c418:u8=107
const c427:u8=82
const c436:u8=9
const c444:u8=0
const c450:u8=0
const c478:u8=64
const c488:u8=2
const c505:u8=33
const c569:u8=253
const c612:u8=102
const c620:u8=61
const c636:u8=203
const c641:u8=33
const c650:u8=13
const c670:u8=94
const c682:u8=0
const c690:u8=0
const c703:u8=201
const c743:u8=32
const c748:u8=165
const c754:u8=33
const c761:u8=0
const c785:u8=22
const c796:u8=136
const c818:u8=202
const c844:u8=171
const c850:u8=175
const c866:u8=123
const c899:u8=252
const c904:u8=250
const c913:u8=34
const c941:u8=60
const c955:u8=23
const c960:u8=23
const c966:u8=29
const c985:u8=193
const c990:u8=144
const c1000:u8=214
const c1012:u8=72
const c1035:u8=26
const c1053:u8=0
const c1073:u8=135
const c1118:u8=203
const c1149:u8=86
const c1162:u8=205
const c1169:u8=224
const c1200:u8=3
const c1206:u8=11
const c1227:u8=65
const c1234:u8=18
const c1256:u8=249
const c1262:u8=1
const c1269:u8=12
const c1295:u8=108
const c1301:u8=16
static RESULT:u8
(= RESULT (+ (- (- (- (- (+ (- 75 (- 119 (+ (- c37 c41) c46))) 213) c58) (- (- (+ (- (- (- 193 c85) (- c93 (+ (- (- (+ (+ 7 c114) c120) c126) 44) 0))) (+ c144 c149)) 70) (- (- c166 (+ (+ c177 5) (+ (- (- (- 233 7) c204) (+ (- (+ (- (+ c225 c230) c236) (- 174 (+ 3 22))) 196) c265)) (- 121 c279)))) (+ 26 c294))) (- (+ (- 129 (- (+ 15 (- (+ 230 0) (- 190 (+ c347 (- 244 (+ 33 (+ 56 c371))))))) c383)) (- (+ 33 169) (- c407 (+ (- c418 87) c427)))) c436))) c444) c450) (- (- (- (- (- 246 (- c478 64)) c488) (+ (+ 3 (- c505 30)) 26)) (- (- 242 (+ (- (- (- (+ 42 (- 222 11)) 0) (- (- c569 (- (- 248 (+ (+ 1 (+ (- 94 93) (- 110 c612))) c620)) (- (- (- c636 c641) 2) c650))) (+ 31 (+ (+ c670 94) (+ c682 (+ c690 0)))))) c703) (+ (+ (- (+ (- (+ (+ (- 196 (- (+ c743 c748) c754)) c761) 196) 211) (+ 7 (+ c785 (- (- c796 53) 61)))) (- (- c818 (- 76 (- (+ (- (+ 28 c844) c850) 74) 49))) c866)) (- (+ (- (+ (+ 6 (+ (+ (- c899 c904) 6) c913)) 49) 90) 39) (+ (+ (- c941 (+ (- (+ c955 c960) c966) (- (- 201 (- c985 c990)) (- c1000 (+ 24 c1012))))) (+ (- (+ (+ c1035 (- 214 53)) c1053) (- (- 248 (- c1073 94)) 20)) 0)) 37))) (- 200 (+ (- (+ (- c1118 (- (+ 100 (+ (- 173 159) c1149)) 38)) c1162) c1169) (+ 22 (+ (+ (- (- (+ (+ c1200 c1206) (+ 29 30)) 4) c1227) c1234) (+ (- (- (- (- c1256 c1262) c1269) 71) 148) (- 144 (- c1295 c1301)))))))))) 54)) 145)))
"#);
}
#[test]
fn const_expr_193() {
    test_const_expr(r#"
const c6:u8=134
const c9:u8=7
const c16:u8=252
const c20:u8=124
static RESULT:u8
(= RESULT (+ (- c6 c9) (- c16 c20)))
"#);
}
#[test]
fn const_expr_194() {
    test_const_expr(r#"
const c26:u8=0
const c52:u8=151
const c78:u8=44
const c85:u8=232
const c99:u8=136
const c119:u8=0
const c127:u8=186
const c154:u8=43
const c165:u8=0
const c185:u8=10
const c203:u8=80
const c215:u8=215
const c230:u8=185
const c236:u8=162
const c251:u8=0
const c266:u8=240
const c271:u8=6
const c279:u8=0
static RESULT:u8
(= RESULT (- (- (+ 42 213) (+ (+ (+ c26 0) (- (+ (- 163 (- (+ c52 0) (+ (- 252 (- (+ (+ c78 (- c85 (+ (- 149 c99) 40))) (+ (+ (+ c119 (- c127 186)) (+ 0 0)) 0)) (+ c154 43))) c165))) 0) (+ (+ (+ c185 11) 0) (+ 26 c203)))) (- c215 (+ (- 238 c230) c236)))) (+ (+ c251 (- 234 (- c266 c271))) c279)))
"#);
}
#[test]
fn const_expr_195() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 36 219))
"#);
}
#[test]
fn const_expr_196() {
    test_const_expr(r#"
const c31:u8=213
const c76:u8=44
const c83:u8=0
const c89:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (+ 42 (- 250 250)) c31) (+ 0 (- 138 138))) (+ (+ 0 (- 72 (- 116 c76))) c83)) c89))
"#);
}
#[test]
fn const_expr_197() {
    test_const_expr(r#"
const c6:u8=31
const c15:u8=3
const c22:u8=88
const c64:u8=149
const c124:u8=154
const c141:u8=119
const c154:u8=180
const c159:u8=4
const c178:u8=46
const c210:u8=188
const c230:u8=166
static RESULT:u8
(= RESULT (+ (+ c6 (+ (+ c15 (- c22 (+ (+ (- (+ 123 123) 244) (- (- (+ 49 c64) 37) (+ (- 67 32) 108))) (+ (- (+ 19 (+ 31 (- 138 (- (- c124 (+ 12 (- (+ c141 120) (- c154 c159)))) 5)))) 99) c178)))) (- (- 231 (- 129 108)) c210))) (- (- (+ 82 c230) 3) 53)))
"#);
}
#[test]
fn const_expr_198() {
    test_const_expr(r#"
const c22:u8=18
const c35:u8=120
const c49:u8=213
static RESULT:u8
(= RESULT (+ (- (+ (- 97 69) (+ c22 (+ 13 (- c35 38)))) (- c49 199)) 128))
"#);
}
#[test]
fn const_expr_199() {
    test_const_expr(r#"
const c12:u8=63
const c16:u8=192
const c37:u8=245
const c41:u8=32
static RESULT:u8
(= RESULT (- (- (- (+ c12 c16) 0) 0) (- 213 (- c37 c41))))
"#);
}
#[test]
fn const_expr_200() {
    test_const_expr(r#"
const c15:u8=68
const c19:u8=0
const c50:u8=46
const c54:u8=46
const c77:u8=154
const c94:u8=9
const c103:u8=6
const c126:u8=222
const c140:u8=181
const c145:u8=64
const c154:u8=16
const c159:u8=101
const c172:u8=31
const c195:u8=26
const c220:u8=8
const c239:u8=43
const c253:u8=31
const c272:u8=212
const c313:u8=63
const c324:u8=5
const c329:u8=33
const c338:u8=76
const c343:u8=77
const c351:u8=0
const c357:u8=17
const c366:u8=193
const c378:u8=86
const c387:u8=120
const c396:u8=29
static RESULT:u8
(= RESULT (+ (+ (+ 17 (+ c15 c19)) (- 245 (+ 12 63))) (- (+ c50 c54) (- (+ (+ 19 (- (- c77 (- 122 (+ (+ c94 (+ 3 c103)) (- (- (- 250 (- c126 (- (+ (- c140 c145) (+ c154 c159)) 18))) c172) 213)))) (- (+ (+ c195 156) 0) (+ (+ (- (+ c220 52) 39) (- (+ c239 (+ (- (+ c253 (- 159 (- 216 c272))) (+ (+ 4 20) 74)) 0)) (- (- (- (+ c313 (+ (+ c324 c329) (+ c338 c343))) c351) c357) (- c366 22)))) c378)))) c387) (+ c396 58)))))
"#);
}
#[test]
fn const_expr_201() {
    test_const_expr(r#"
const c38:u8=186
const c44:u8=32
const c52:u8=26
const c62:u8=142
const c66:u8=0
const c92:u8=254
const c102:u8=251
const c164:u8=8
const c174:u8=132
const c183:u8=208
static RESULT:u8
(= RESULT (+ (- (+ (+ (- (- (- 252 2) 11) (+ 46 c38)) c44) (+ c52 (- (+ c62 c66) (+ (+ (- (+ 17 (- (- c92 0) (- c102 100))) (+ 40 80)) (+ (- (- 104 9) 95) (- 125 123))) (+ 2 c164))))) c174) (- c183 16)))
"#);
}
#[test]
fn const_expr_202() {
    test_const_expr(r#"
const c9:u8=31
const c20:u8=200
const c24:u8=72
static RESULT:u8
(= RESULT (+ (- (+ c9 160) (- c20 c24)) 192))
"#);
}
#[test]
fn const_expr_203() {
    test_const_expr(r#"
const c25:u8=217
const c44:u8=132
const c68:u8=253
const c93:u8=12
const c100:u8=30
const c109:u8=22
const c114:u8=66
const c142:u8=213
const c151:u8=0
const c156:u8=3
const c166:u8=254
const c188:u8=236
const c201:u8=53
const c212:u8=8
const c238:u8=18
const c255:u8=103
const c262:u8=12
const c276:u8=55
const c320:u8=93
static RESULT:u8
(= RESULT (+ (- (- (- (+ (- 46 10) c25) (+ 2 (+ (- (- c44 (+ 1 (- (- (- (- (- c68 (- (+ 72 145) (- 226 c93))) c100) (+ c109 c114)) (+ (- (- (+ (+ 5 30) c142) (+ c151 c156)) (- c166 23)) (- (+ (- (- c188 1) 222) c201) (+ 2 c212)))) 55))) (+ (+ (+ 2 c238) (- 40 40)) c255)) c262))) 0) (+ c276 56)) (- (- 220 (+ 21 (+ (+ 2 10) (+ (- c320 76) 35)))) 7)))
"#);
}
#[test]
fn const_expr_204() {
    test_const_expr(r#"
const c51:u8=7
const c55:u8=24
const c97:u8=50
const c133:u8=0
const c144:u8=0
const c149:u8=1
const c172:u8=123
const c177:u8=100
const c186:u8=9
const c211:u8=177
const c226:u8=48
const c239:u8=3
const c259:u8=123
const c277:u8=95
const c289:u8=0
const c300:u8=180
const c330:u8=179
const c366:u8=152
const c381:u8=30
const c397:u8=16
const c404:u8=48
const c447:u8=206
const c452:u8=4
const c458:u8=136
const c490:u8=18
const c495:u8=112
const c513:u8=61
const c540:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ 42 (- 215 (+ 0 2))) (+ (- (- 249 (+ (+ c51 c55) 186)) (- (+ 251 0) (- (- (- (- (- (+ c97 202) (- (+ 53 (+ 27 81)) (+ 160 c133))) (+ c144 c149)) 11) (- 93 (+ (- c172 c177) (+ c186 (+ (+ 12 (+ 0 (+ (- c211 177) 0))) c226))))) (+ c239 (+ 8 8))))) (- c259 (- (- 218 0) c277)))) (+ c289 (- (- c300 132) (+ (- (+ 9 (- (+ (- c330 (- (+ (- (+ (- (- 229 105) 99) c366) (- (- (+ c381 186) (- 29 c397)) c404)) (+ 28 87)) (+ (- (- (- 226 0) (- (- c447 c452) c458)) 140) (- 141 (- 177 (- (+ c490 c495) 34)))))) (+ c513 62)) 225)) 17) 36)))) c540))
"#);
}
#[test]
fn const_expr_205() {
    test_const_expr(r#"
const c12:u8=28
const c37:u8=70
const c51:u8=156
const c69:u8=0
const c89:u8=29
const c120:u8=112
const c132:u8=160
const c137:u8=8
const c152:u8=3
const c168:u8=142
const c179:u8=116
const c185:u8=145
const c194:u8=0
const c205:u8=85
const c210:u8=85
const c219:u8=2
const c260:u8=79
const c278:u8=69
const c283:u8=140
const c289:u8=209
const c295:u8=0
const c307:u8=10
const c319:u8=11
const c335:u8=249
const c354:u8=0
static RESULT:u8
(= RESULT (- (+ (- (+ c12 (+ (+ (+ (- (+ (+ (- c37 (- 220 (- c51 (+ (+ (+ 0 0) c69) 1)))) (- (- (+ c89 148) (- (+ (+ 13 (+ 4 10)) c120) (- (- c132 c137) (+ 11 (+ c152 21))))) (- c168 11))) c179) c185) (+ c194 (+ (- c205 c210) (+ c219 0)))) 14) (+ (+ (- 17 (+ 8 (+ (- 79 c260) (+ (+ (- (+ c278 c283) c289) c295) 0)))) c307) (+ (+ c319 66) (- 249 c335))))) 13) 128) c354))
"#);
}
#[test]
fn const_expr_206() {
    test_const_expr(r#"
const c27:u8=74
const c31:u8=65
const c36:u8=54
const c44:u8=209
const c48:u8=17
const c78:u8=250
const c85:u8=251
const c94:u8=4
const c110:u8=126
const c115:u8=126
const c147:u8=49
const c155:u8=78
const c160:u8=28
const c178:u8=12
const c209:u8=96
const c222:u8=43
const c252:u8=238
const c257:u8=66
const c284:u8=44
const c308:u8=157
const c327:u8=199
const c339:u8=44
const c364:u8=0
const c376:u8=57
const c385:u8=20
const c393:u8=17
const c422:u8=15
const c433:u8=78
const c450:u8=197
const c469:u8=54
const c478:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (- (- (+ (+ (- c27 c31) c36) (- c44 c48)) (+ (- (+ (+ (- (+ (+ (- c78 (- c85 (+ 1 c94))) (- (+ (+ c110 c115) (- 32 32)) (- 243 3))) (+ c147 (- c155 c160))) 111) (+ 2 c178)) 72) (- (+ 123 (+ (- 113 c209) (- 150 c222))) 157)) 0)) (- 31 (- (- c252 c257) (+ (- (+ (- 95 (+ 44 c284)) (+ 7 (+ (- (+ (- c308 125) (- (+ 49 c327) (+ 10 c339))) (- 236 (- (+ 126 c364) (+ 56 c376)))) c385))) c393) 121)))) 0) (- (- (+ (+ c422 (+ 13 c433)) 0) 53) (- c450 (- (+ 33 165) c469)))) c478) 0))
"#);
}
#[test]
fn const_expr_207() {
    test_const_expr(r#"
const c9:u8=21
static RESULT:u8
(= RESULT (+ (+ (+ c9 (+ 53 53)) 0) (+ 42 (+ 17 (- 82 13)))))
"#);
}
#[test]
fn const_expr_208() {
    test_const_expr(r#"
static RESULT:u8
(= RESULT (+ 127 128))
"#);
}
#[test]
fn const_expr_209() {
    test_const_expr(r#"
const c16:u8=2
const c32:u8=74
const c50:u8=239
const c60:u8=4
static RESULT:u8
(= RESULT (+ 51 (- 220 (+ c16 (- 24 (+ (- c32 69) (+ (- 240 c50) (+ 0 c60))))))))
"#);
}
#[test]
fn const_expr_210() {
    test_const_expr(r#"
const c34:u8=28
const c42:u8=0
const c61:u8=9
const c65:u8=9
const c81:u8=12
const c91:u8=178
const c110:u8=169
const c118:u8=243
const c136:u8=28
const c146:u8=10
static RESULT:u8
(= RESULT (+ (+ (- (- (- (+ (+ (+ (+ (- 118 c34) 0) c42) 90) 0) 72) (+ c61 c65)) (- 118 (+ c81 (- (- c91 117) 24)))) (- c110 (- c118 116))) (- (+ c136 174) c146)))
"#);
}
#[test]
fn const_expr_211() {
    test_const_expr(r#"
const c43:u8=254
const c55:u8=2
const c64:u8=10
const c68:u8=31
const c74:u8=19
const c83:u8=0
const c94:u8=181
const c114:u8=0
const c119:u8=0
const c144:u8=12
const c154:u8=10
const c172:u8=61
const c179:u8=5
const c188:u8=156
const c200:u8=0
const c208:u8=0
const c244:u8=250
const c260:u8=93
const c268:u8=0
const c372:u8=1
const c380:u8=1
const c390:u8=132
const c414:u8=15
const c462:u8=132
const c471:u8=11
const c481:u8=202
const c494:u8=11
const c506:u8=135
const c515:u8=35
const c546:u8=2
const c555:u8=146
const c568:u8=65
const c578:u8=148
const c586:u8=82
const c591:u8=45
const c608:u8=3
const c622:u8=171
const c627:u8=8
const c633:u8=30
const c651:u8=200
const c659:u8=180
const c668:u8=126
const c681:u8=0
const c688:u8=197
const c698:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (- (- (+ (+ 15 48) (- (- (- (- c43 0) (+ 0 c55)) (+ c64 c68)) c74)) 0) c83) (+ (- c94 181) (+ (+ 0 (+ c114 c119)) (+ (+ (- (- (+ (+ c144 (+ 2 c154)) 99) (- 179 c172)) c179) (- c188 (+ 156 c200))) c208)))) (- (+ (- (+ (- (+ (- (- (- c244 39) 73) (+ c260 (+ c268 0))) 182) 214) 84) (+ (- (- (- 200 2) (- 92 79)) (+ 30 (+ (- (- 168 34) (- (+ (- (- 54 (+ 0 3)) (+ c372 (+ c380 5))) c390) (+ (+ 5 (- (- 163 c414) (+ 20 101))) 131))) (- (+ (- (+ (+ (- 143 c462) (+ c471 11)) c481) (- 219 c494)) 108) c506)))) c515)) (+ (- (+ (- (+ (- (- 76 c546) 1) c555) (- 173 c568)) (- c578 (- c586 c591))) (- (+ (+ c608 (- (- (- c622 c627) c633) 112)) (+ (- c651 (+ c659 0)) c668)) 117)) c681)) c688)) 0) c698))
"#);
}
#[test]
fn const_expr_212() {
    test_const_expr(r#"
const c15:u8=7
const c73:u8=9
const c77:u8=9
const c84:u8=30
const c98:u8=0
const c103:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (+ c15 (+ (+ 2 (+ (+ (+ 0 (+ (- 50 50) 1)) (+ (+ 0 2) 0)) (- c73 c77))) c84)) 213) 0) c98) c103))
"#);
}
#[test]
fn const_expr_213() {
    test_const_expr(r#"
const c3:u8=127
static RESULT:u8
(= RESULT (+ c3 128))
"#);
}
#[test]
fn const_expr_214() {
    test_const_expr(r#"
const c9:u8=253
const c66:u8=46
const c79:u8=30
const c83:u8=31
const c97:u8=31
const c101:u8=62
const c119:u8=63
const c135:u8=215
const c146:u8=230
const c156:u8=26
const c169:u8=41
const c198:u8=191
static RESULT:u8
(= RESULT (+ 51 (- c9 (+ 12 (- 185 (- 165 (- (+ 28 0) (+ 2 (+ 9 (- (- (- (+ c66 (- (+ (+ c79 c83) (+ 30 (+ c97 c101))) (- (- 244 c119) 137))) (- c135 (- (- c146 (+ 5 c156)) (- (+ c169 206) 229)))) 103) (+ (- c198 175) 33)))))))))))
"#);
}
#[test]
fn const_expr_215() {
    test_const_expr(r#"
const c9:u8=42
const c25:u8=0
static RESULT:u8
(= RESULT (- (+ (+ c9 213) 0) (+ 0 c25)))
"#);
}
#[test]
fn const_expr_216() {
    test_const_expr(r#"
const c15:u8=27
const c29:u8=30
const c46:u8=154
const c50:u8=109
const c75:u8=49
const c82:u8=16
const c131:u8=0
const c139:u8=45
const c175:u8=56
const c182:u8=159
const c191:u8=44
const c210:u8=214
const c216:u8=4
const c222:u8=16
const c228:u8=21
const c251:u8=114
const c265:u8=23
const c271:u8=214
static RESULT:u8
(= RESULT (- (+ (- (- (+ c15 163) 124) c29) (+ (- (+ (- c46 c50) 90) 81) 165)) (+ (- c75 (+ c82 33)) (+ (- (- (- (+ (+ (+ (+ (- (- (+ 230 (+ c131 (- c139 45))) (- (+ (- (+ 146 0) (+ 11 c175)) c182) (+ c191 (- 218 40)))) c210) c216) c222) c228) (- 211 6)) (- (- c251 85) 20)) c265) c271) 0))))
"#);
}
#[test]
fn const_expr_217() {
    test_const_expr(r#"
const c13:u8=128
static RESULT:u8
(= RESULT (- (- (+ 127 c13) 0) 0))
"#);
}
#[test]
fn const_expr_218() {
    test_const_expr(r#"
const c12:u8=240
const c24:u8=86
const c28:u8=56
static RESULT:u8
(= RESULT (- (+ 51 (- c12 (+ 6 (- c24 c28)))) (+ 0 (+ 0 0))))
"#);
}
#[test]
fn const_expr_219() {
    test_const_expr(r#"
const c15:u8=0
static RESULT:u8
(= RESULT (- (+ 127 128) c15))
"#);
}
#[test]
fn const_expr_220() {
    test_const_expr(r#"
const c6:u8=127
const c9:u8=128
static RESULT:u8
(= RESULT (- (+ c6 c9) 0))
"#);
}
#[test]
fn const_expr_221() {
    test_const_expr(r#"
const c6:u8=85
const c18:u8=251
const c42:u8=67
const c71:u8=28
const c81:u8=113
const c91:u8=1
const c130:u8=176
const c176:u8=41
const c181:u8=0
const c206:u8=251
const c229:u8=122
const c242:u8=166
const c251:u8=200
const c262:u8=8
const c273:u8=182
const c281:u8=7
const c289:u8=218
const c294:u8=218
const c305:u8=209
const c313:u8=192
const c318:u8=141
const c353:u8=10
const c402:u8=196
const c409:u8=0
static RESULT:u8
(= RESULT (- (+ c6 (+ (- (- c18 (- 179 (- (+ (- 235 c42) (- (- (+ (- (- (+ 42 (+ c71 (+ 28 c81))) (+ c91 (- (+ 14 57) (+ 22 44)))) 110) 96) c130) 15)) (- 43 (+ (- (- 199 (- (+ (- 217 (+ c176 c181)) 0) (- (- (+ 58 (- c206 135)) (+ 1 9)) (- c229 105)))) c242) (- c251 (+ (+ c262 (- (- c273 (+ c281 (- c289 c294))) (- c305 (- c313 c318)))) (- 211 (+ 14 (+ 7 (+ (+ 2 c353) 24))))))))))) (- (+ 82 (+ (- 118 77) 125)) c402)) c409)) 0))
"#);
}
#[test]
fn const_expr_222() {
    test_const_expr(r#"
const c15:u8=181
const c20:u8=80
const c25:u8=95
const c46:u8=149
const c66:u8=0
const c85:u8=37
const c89:u8=21
const c103:u8=196
const c108:u8=185
const c123:u8=76
const c137:u8=244
const c142:u8=62
const c163:u8=0
const c172:u8=17
const c186:u8=192
const c206:u8=176
const c235:u8=124
const c244:u8=37
const c270:u8=23
const c286:u8=0
const c294:u8=2
const c316:u8=225
const c330:u8=6
const c335:u8=37
const c347:u8=0
const c377:u8=51
const c383:u8=5
static RESULT:u8
(= RESULT (+ (- (- (+ 45 c15) c20) c25) (+ (- 130 (- (- c46 (- 178 129)) (+ c66 (- (+ 63 (+ (- c85 c89) (- (+ (- c103 c108) (- (+ (+ c123 (- (+ (- c137 c142) (+ 0 0)) 106)) c163) (+ c172 (- (+ 31 c186) 134)))) (+ (- c206 167) (- (- (+ 135 (- (- c235 87) c244)) (- 199 (- (+ 13 (+ c270 46)) (+ (+ c286 0) c294)))) (+ (- 156 (- c316 (+ 42 (+ c330 c335)))) (+ c347 (+ 0 0)))))))) (- (- 179 c377) c383))))) 170)))
"#);
}
#[test]
fn const_expr_223() {
    test_const_expr(r#"
const c57:u8=22
const c61:u8=22
static RESULT:u8
(= RESULT (+ 42 (- 216 (+ (- (+ 16 (- (- 251 0) 187)) 80) (+ (- (+ c57 c61) 43) (- (+ 49 (- (- 250 7) (+ 6 41))) 243))))))
"#);
}
#[test]
fn const_expr_224() {
    test_const_expr(r#"
const c9:u8=19
const c30:u8=109
const c43:u8=236
const c47:u8=4
const c55:u8=0
const c62:u8=14
const c69:u8=2
const c85:u8=190
const c101:u8=140
const c109:u8=144
const c114:u8=4
const c124:u8=155
const c132:u8=168
const c146:u8=149
const c157:u8=26
const c162:u8=0
const c175:u8=129
const c202:u8=7
const c220:u8=16
const c226:u8=196
const c232:u8=148
const c244:u8=34
const c255:u8=4
const c269:u8=27
const c295:u8=100
const c301:u8=78
const c329:u8=18
const c348:u8=185
const c360:u8=2
const c365:u8=2
const c376:u8=3
const c397:u8=203
const c431:u8=133
const c445:u8=106
const c451:u8=61
const c459:u8=0
static RESULT:u8
(= RESULT (+ (- (+ c9 (+ (+ 58 (- (- (+ c30 (- (- (- c43 c47) (+ c55 (- c62 (+ c69 9)))) (+ (- c85 (+ (+ (+ (- c101 (- c109 c114)) (- c124 (- c132 17))) (- c146 (+ (+ c157 c162) 106))) c175)) (- (+ (+ (+ 8 (+ (- c202 5) (+ 2 4))) c220) c226) c232)))) (+ c244 (+ (+ c255 (+ (- (+ c269 167) (+ (+ (+ (- 119 c295) c301) (- (- 201 4) 100)) (- c329 (- (- 253 50) c348)))) (+ c360 c365))) (+ c376 24)))) (+ (- (- c397 (- 114 (- (+ (+ 13 69) (+ 33 c431)) 201))) c445) c451))) c459)) 8) 128))
"#);
}
#[test]
fn const_expr_225() {
    test_const_expr(r#"
const c15:u8=42
const c31:u8=15
const c35:u8=15
const c67:u8=251
const c74:u8=0
const c84:u8=0
const c91:u8=233
const c95:u8=232
const c106:u8=248
const c121:u8=0
const c145:u8=23
const c150:u8=69
const c170:u8=42
const c183:u8=7
const c218:u8=201
const c224:u8=210
const c230:u8=17
const c246:u8=233
const c264:u8=211
const c278:u8=182
const c287:u8=21
const c299:u8=109
const c307:u8=194
const c342:u8=4
const c350:u8=1
const c363:u8=0
const c390:u8=0
const c410:u8=0
static RESULT:u8
(= RESULT (- (- (+ 85 (+ c15 128)) (+ (- c31 c35) (+ (+ (+ (+ (+ (+ (+ (- (- c67 (+ c74 (+ (+ c84 (- c91 c95)) 2))) c106) 0) 0) 0) c121) (+ (- (+ 12 (- (+ c145 c150) (+ 19 0))) (+ c170 43)) (- c183 7))) (+ (- (- (- (- (+ (- 236 c218) c224) c230) 14) (+ (- c246 (- 243 (- (- c264 (- (- (- c278 27) c287) (+ 18 c299))) c307))) 1)) (+ (- (+ (+ (+ (+ 2 (+ c342 (+ c350 9))) 0) c363) 98) 61) (- 232 72))) c390)) (- 40 40)))) c410))
"#);
}
#[test]
fn const_expr_226() {
    test_const_expr(r#"
const c18:u8=85
const c22:u8=170
const c27:u8=0
const c47:u8=180
const c57:u8=53
const c86:u8=124
const c98:u8=8
const c111:u8=0
const c131:u8=48
const c154:u8=126
const c167:u8=2
const c195:u8=53
const c220:u8=54
const c243:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (- (+ c18 c22) c27) 0) (- (+ 45 (+ c47 (- 53 c57))) (- 245 (- (- (- (+ 61 c86) 86) (+ c98 (+ (+ (+ c111 (- (+ 78 (- (+ c131 (- 235 41)) (- (+ c154 127) (+ c167 9)))) 77)) 7) (- (+ (+ c195 (- 146 (- 194 48))) c220) 65)))) 21)))) 0) c243))
"#);
}
#[test]
fn const_expr_227() {
    test_const_expr(r#"
const c42:u8=129
const c49:u8=108
const c60:u8=74
const c67:u8=20
const c97:u8=205
const c107:u8=4
const c121:u8=236
const c136:u8=116
const c143:u8=18
const c150:u8=165
const c164:u8=123
const c179:u8=180
const c184:u8=30
const c198:u8=119
const c209:u8=11
const c214:u8=56
const c220:u8=18
const c227:u8=24
const c235:u8=4
const c256:u8=178
const c279:u8=0
const c289:u8=104
const c322:u8=0
const c327:u8=5
const c334:u8=36
const c348:u8=38
const c354:u8=90
const c368:u8=107
const c408:u8=209
const c413:u8=204
const c441:u8=20
const c463:u8=111
const c488:u8=82
const c520:u8=4
const c534:u8=0
const c566:u8=138
const c575:u8=146
const c580:u8=91
const c601:u8=0
const c628:u8=212
const c639:u8=247
const c649:u8=98
const c711:u8=50
const c718:u8=168
const c730:u8=233
const c735:u8=21
const c741:u8=182
const c754:u8=178
const c767:u8=252
const c772:u8=48
const c812:u8=0
const c824:u8=2
const c833:u8=9
const c859:u8=0
const c871:u8=249
const c880:u8=32
const c894:u8=0
const c899:u8=0
static RESULT:u8
(= RESULT (+ (- (+ (+ (+ (+ (- (- 252 138) (+ (- (+ c42 0) c49) (+ 14 c60))) c67) (+ (+ (- (- (+ 11 33) (- c97 (+ (+ c107 (+ (- (+ c121 0) (+ 115 c136)) c143)) c150))) (- (- c164 (- 197 (- c179 c184))) (- (- c198 (- (+ c209 c214) c220)) c227))) c235) (+ (- (+ (- (- c256 (- (- 246 (+ 0 (+ c279 0))) c289)) (- (- 229 (- (- (- 189 (+ c322 c327)) c334) (+ (+ 7 c348) c354))) 206)) c368) (+ (- (+ (- (+ (- (- (- 250 7) (- c408 c413)) 118) (+ 0 0)) 37) (+ c441 63)) (- 204 56)) c463)) (- (+ (+ (- (- (+ c488 (- (+ 31 (+ 22 136)) (+ (+ c520 20) (+ 0 c534)))) 108) 107) (+ (+ (- 149 c566) (- c575 c580)) (- 237 105))) c601) (+ (+ (+ (- (+ 17 (- c628 (+ (- c639 223) c649))) 106) (- (- (+ 30 123) (- 240 133)) (+ (- (+ 29 (- 199 c711)) c718) (- (- c730 c735) c741)))) 28) c754))))) (- c767 c772)) (- (+ (+ (+ (+ (+ (+ 0 (- (+ 237 c812) 237)) c824) (+ c833 (- 33 33))) 23) 172) c859) (- (- c871 11) c880))) 0) (+ c894 c899)))
"#);
}
#[test]
fn const_expr_228() {
    test_const_expr(r#"
const c24:u8=187
const c34:u8=96
const c51:u8=0
const c58:u8=108
const c62:u8=108
const c81:u8=23
const c89:u8=14
const c99:u8=195
const c129:u8=19
const c134:u8=12
const c140:u8=35
const c146:u8=0
const c153:u8=38
const c162:u8=12
const c167:u8=60
const c178:u8=0
const c196:u8=0
const c201:u8=4
const c213:u8=21
const c230:u8=133
const c260:u8=43
const c278:u8=230
const c304:u8=76
const c316:u8=37
const c350:u8=207
const c361:u8=234
const c389:u8=130
const c402:u8=0
const c415:u8=29
const c428:u8=129
const c445:u8=36
const c450:u8=217
const c456:u8=1
const c465:u8=179
const c486:u8=25
const c499:u8=12
const c504:u8=76
const c512:u8=136
static RESULT:u8
(= RESULT (- (- (+ 85 (+ 34 (+ (- c24 (+ (- c34 (- (+ 212 (+ c51 (- c58 c62))) 196)) (- (+ c81 71) c89))) (- c99 (+ (- (+ (+ 3 7) (+ (+ (- c129 c134) c140) c146)) c153) (+ c162 c167)))))) c178) (- (+ (+ (+ c196 c201) (- 29 c213)) (- (- 222 c230) 24)) (- (- 209 (+ (+ (- c260 (- 180 (- (- c278 16) (- (- 172 (- 143 c304)) (+ 9 c316))))) (- 5 (+ (+ 0 0) 5))) (- c350 (- (- c361 (- (+ (+ (+ (+ 0 (+ (- c389 130) (+ c402 0))) (- c415 26)) (- c428 (- (- (- (+ c445 c450) c456) (- c465 163)) 119))) (+ c486 51)) (+ c499 c504))) c512)))) 2))))
"#);
}
#[test]
fn const_expr_229() {
    test_const_expr(r#"
const c6:u8=170
static RESULT:u8
(= RESULT (+ 85 c6))
"#);
}
#[test]
fn const_expr_230() {
    test_const_expr(r#"
const c18:u8=12
const c40:u8=66
const c50:u8=73
const c66:u8=99
const c96:u8=11
const c107:u8=157
const c115:u8=0
const c138:u8=102
const c143:u8=101
static RESULT:u8
(= RESULT (- (- (+ (- (+ (+ c18 (- (- (- (+ 49 (+ c40 133)) c50) 118) (- (+ c66 0) 66))) 219) (+ 0 0)) (- c96 (- 168 c107))) c115) (+ (- (+ 0 1) (- c138 c143)) (+ 0 0))))
"#);
}
#[test]
fn const_expr_231() {
    test_const_expr(r#"
const c19:u8=163
const c24:u8=24
const c61:u8=48
const c68:u8=29
const c87:u8=221
const c100:u8=243
const c118:u8=48
const c127:u8=28
const c144:u8=66
const c158:u8=209
const c172:u8=0
const c184:u8=188
const c189:u8=8
const c219:u8=8
static RESULT:u8
(= RESULT (- (- (+ (+ (- 175 c19) c24) (- (+ (- (- 247 27) (- (- (- (+ c61 (+ c68 116)) (+ (- (- c87 (- (- (- c100 5) (- (- 219 c118) (+ c127 115))) 56)) c144) (+ 8 (- c158 (+ (+ 29 c172) (+ (- c184 c189) (+ (+ 0 0) 0))))))) 14) c219)) 176) 15)) 0) 0))
"#);
}
#[test]
fn const_expr_232() {
    test_const_expr(r#"
const c3:u8=42
static RESULT:u8
(= RESULT (+ c3 213))
"#);
}
#[test]
fn const_expr_233() {
    test_const_expr(r#"
const c9:u8=51
const c27:u8=28
const c31:u8=85
const c44:u8=0
const c54:u8=129
const c58:u8=129
static RESULT:u8
(= RESULT (- (- (+ c9 204) (- 113 (+ c27 c31))) (+ (+ c44 0) (- c54 c58))))
"#);
}
#[test]
fn const_expr_234() {
    test_const_expr(r#"
const c15:u8=210
const c22:u8=83
const c33:u8=221
const c37:u8=93
const c58:u8=230
const c62:u8=1
const c67:u8=45
const c84:u8=49
const c96:u8=176
const c100:u8=171
const c130:u8=14
const c136:u8=64
const c142:u8=138
const c148:u8=29
const c188:u8=176
const c224:u8=4
const c243:u8=0
const c251:u8=0
const c261:u8=32
const c275:u8=18
const c286:u8=137
const c291:u8=119
const c302:u8=0
const c314:u8=0
const c319:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ (- c15 (+ c22 0)) (- c33 c37)) (+ (+ (- (- (- c58 c62) c67) (- (- (- (+ c84 200) (- c96 c100)) 50) (- (+ (- (- (- 225 c130) c136) c142) c148) (+ (+ (- 73 (- 109 (+ 18 (+ (- (- c188 137) 36) (+ (- 143 136) 8))))) c224) 24)))) (+ (+ c243 0) c251)) (- c261 32))) (- c275 (+ (- c286 c291) (+ 0 c302)))) (+ c314 c319)))
"#);
}
#[test]
fn const_expr_235() {
    test_const_expr(r#"
const c9:u8=49
const c18:u8=38
const c41:u8=10
const c52:u8=94
const c56:u8=94
const c94:u8=6
const c116:u8=186
const c124:u8=24
const c135:u8=4
const c146:u8=147
const c154:u8=55
const c166:u8=198
const c187:u8=120
const c195:u8=84
const c207:u8=17
const c215:u8=87
const c229:u8=46
const c247:u8=4
const c278:u8=15
static RESULT:u8
(= RESULT (+ (- (+ c9 (- (+ c18 (+ (+ (+ (- (- 162 c41) (- (+ c52 c56) 38)) 2) (+ 3 (+ 18 (- 196 (- 202 c94))))) (- (- 183 (- c116 (+ c124 (+ (+ c135 (- (- c146 (+ c154 (- 253 c166))) (- (+ 48 97) c187))) c195)))) (+ c207 (- c215 (+ (- (+ c229 139) 178) (+ c247 (- (+ 27 112) 115)))))))) c278)) 107) 213))
"#);
}
#[test]
fn const_expr_236() {
    test_const_expr(r#"
const c3:u8=63
static RESULT:u8
(= RESULT (+ c3 (+ (+ 16 32) 144)))
"#);
}
#[test]
fn const_expr_237() {
    test_const_expr(r#"
const c3:u8=127
const c6:u8=128
static RESULT:u8
(= RESULT (+ c3 c6))
"#);
}
#[test]
fn const_expr_238() {
    test_const_expr(r#"
const c32:u8=160
const c37:u8=240
const c68:u8=27
const c119:u8=126
const c133:u8=177
const c160:u8=217
const c172:u8=59
const c180:u8=30
const c188:u8=45
const c218:u8=27
const c232:u8=108
const c248:u8=0
static RESULT:u8
(= RESULT (- (- (- (- (+ 51 204) (- (+ 80 c32) c37)) 0) (- (- (+ (- (- 247 (- c68 (+ 27 0))) 27) (+ (- 104 104) 0)) (+ (- (+ 125 c119) 234) (- c133 (+ (- 89 (+ (+ (- 223 c160) (- (+ c172 (+ c180 (+ c188 (+ (- 13 4) 36)))) 158)) c218)) (- 180 c232))))) 133)) c248))
"#);
}
#[test]
fn const_expr_239() {
    test_const_expr(r#"
const c9:u8=51
const c26:u8=150
const c40:u8=84
const c47:u8=10
const c51:u8=20
const c87:u8=59
const c94:u8=156
const c98:u8=38
const c117:u8=254
const c125:u8=0
const c133:u8=0
const c141:u8=47
const c152:u8=163
const c173:u8=0
const c181:u8=2
const c189:u8=0
const c210:u8=5
const c218:u8=165
const c223:u8=155
const c241:u8=94
const c269:u8=3
const c301:u8=51
const c306:u8=35
const c312:u8=32
const c340:u8=246
const c345:u8=4
const c354:u8=175
const c379:u8=202
const c385:u8=188
const c397:u8=31
const c402:u8=186
const c408:u8=73
const c424:u8=253
const c432:u8=0
const c444:u8=87
const c456:u8=24
const c461:u8=48
const c499:u8=43
const c528:u8=13
const c543:u8=27
const c558:u8=4
const c569:u8=115
const c584:u8=35
const c592:u8=195
const c624:u8=22
const c636:u8=193
const c645:u8=2
const c653:u8=2
const c671:u8=176
const c688:u8=0
static RESULT:u8
(= RESULT (- (+ (+ c9 204) (- (+ (- c26 (- 169 (- c40 (+ c47 c51)))) (- (+ (- 224 (+ 55 110)) (+ c87 (- c94 c98))) (+ (- (- (- c117 (+ c125 (+ c133 (- c141 (+ (- c152 157) 40))))) (+ c173 (+ c181 (+ c189 (- 143 (+ (+ (+ c210 (- c218 c223)) (+ 10 22)) c241)))))) (- (- (+ 50 201) c269) (- 221 207))) (- (+ (+ (- c301 c306) c312) (- (- (+ 115 (- (- (- c340 c345) (- c354 85)) (- (+ (- (+ 33 c379) c385) (- (+ c397 c402) c408)) (- (+ (- c424 9) c432) (+ (- c444 70) (+ c456 c461)))))) (- 139 (+ (- 124 (- 242 (+ c499 (- 221 134)))) (+ 10 (+ c528 (+ 6 (+ 6 c543))))))) (+ c558 18))) c569)))) (- (+ c584 (- c592 (+ (- (+ (+ 49 (+ 16 (+ 11 c624))) 99) c636) (+ c645 (+ c653 (- (+ 92 92) c671)))))) 35))) c688))
"#);
}
#[test]
fn const_expr_240() {
    test_const_expr(r#"
const c25:u8=6
const c43:u8=39
const c62:u8=245
const c75:u8=9
const c106:u8=148
const c118:u8=0
const c125:u8=0
static RESULT:u8
(= RESULT (+ (- (- 177 0) (+ (- (+ c25 12) 5) (- 118 c43))) (+ 24 (+ (- c62 (+ (+ (+ c75 (+ 10 30)) (- (+ (+ 18 55) c106) 171)) c118)) c125))))
"#);
}
#[test]
fn const_expr_241() {
    test_const_expr(r#"
const c18:u8=30
const c36:u8=5
const c49:u8=172
const c69:u8=0
const c84:u8=0
const c102:u8=0
const c129:u8=177
const c142:u8=140
const c151:u8=193
const c159:u8=22
const c195:u8=160
const c204:u8=31
const c215:u8=3
const c236:u8=249
const c247:u8=188
const c259:u8=201
const c284:u8=0
static RESULT:u8
(= RESULT (- (+ (- (+ (- (+ c18 (+ 10 21)) (+ c36 (- (+ (- c49 (+ (- (+ 136 (+ c69 (+ (+ (+ 0 c84) 0) (+ 0 (+ 0 c102))))) (- 174 56)) 95)) c129) 226))) c142) (- c151 (+ c159 112))) (+ (- 164 132) (- (+ 31 c195) (+ c204 (+ (+ c215 (+ (- 84 80) (- c236 (+ 47 c247)))) (- c259 (+ 31 127))))))) (+ c284 0)))
"#);
}
#[test]
fn const_expr_242() {
    test_const_expr(r#"
const c6:u8=42
const c9:u8=213
const c18:u8=0
static RESULT:u8
(= RESULT (- (+ c6 c9) (+ 0 c18)))
"#);
}
#[test]
fn const_expr_243() {
    test_const_expr(r#"
const c12:u8=51
const c21:u8=0
const c29:u8=0
static RESULT:u8
(= RESULT (- (- (- (+ c12 204) c21) (+ c29 0)) 0))
"#);
}
#[test]
fn const_expr_244() {
    test_const_expr(r#"
const c17:u8=0
static RESULT:u8
(= RESULT (- (- (+ 42 213) c17) 0))
"#);
}
#[test]
fn const_expr_245() {
    test_const_expr(r#"
const c6:u8=36
const c14:u8=0
static RESULT:u8
(= RESULT (- (+ c6 219) c14))
"#);
}
#[test]
fn const_expr_246() {
    test_const_expr(r#"
const c26:u8=213
static RESULT:u8
(= RESULT (+ 63 (+ (- (+ 47 190) (- c26 8)) 160)))
"#);
}
#[test]
fn const_expr_247() {
    test_const_expr(r#"
const c19:u8=94
const c56:u8=74
const c60:u8=148
const c68:u8=31
const c84:u8=169
const c88:u8=1
const c96:u8=24
const c112:u8=3
const c119:u8=21
const c132:u8=0
const c149:u8=0
const c158:u8=77
const c163:u8=5
const c173:u8=5
const c190:u8=251
const c198:u8=5
const c203:u8=21
const c237:u8=1
const c253:u8=191
const c279:u8=166
const c300:u8=174
const c310:u8=11
const c324:u8=74
const c332:u8=173
static RESULT:u8
(= RESULT (+ (- (- (- 238 (- c19 (+ (+ (- (+ (+ (+ (+ (+ (+ (- (+ c56 c60) (+ c68 191)) (- (- c84 c88) (+ c96 144))) (+ 0 c112)) c119) 50) (+ c132 (- 20 20))) c149) (- c158 c163)) (+ c173 (+ (- (- (- c190 (+ c198 c203)) (+ (+ (- 111 111) (+ 0 0)) c237)) (- (+ 47 c253) (+ 2 (+ (+ (- (+ 33 c279) 199) (+ (- 174 c300) 2)) c310)))) 4))) c324))) c332) 15) 213))
"#);
}
#[test]
fn const_expr_248() {
    test_const_expr(r#"
const c6:u8=97
const c13:u8=204
static RESULT:u8
(= RESULT (+ (- c6 46) c13))
"#);
}
#[test]
fn const_expr_249() {
    test_const_expr(r#"
const c15:u8=42
const c19:u8=128
const c33:u8=0
const c50:u8=8
const c55:u8=245
static RESULT:u8
(= RESULT (- (- (+ 85 (+ c15 c19)) (+ 0 (+ c33 (+ (- (- 253 c50) c55) 0)))) 0))
"#);
}
#[test]
fn const_expr_250() {
    test_const_expr(r#"
const c14:u8=173
const c18:u8=83
const c23:u8=46
const c61:u8=204
const c73:u8=2
const c86:u8=116
const c106:u8=198
const c124:u8=163
const c134:u8=0
const c152:u8=248
const c169:u8=190
const c174:u8=2
const c184:u8=213
const c200:u8=26
const c231:u8=56
const c265:u8=5
const c280:u8=0
const c285:u8=1
const c291:u8=8
const c306:u8=1
const c321:u8=7
static RESULT:u8
(= RESULT (+ (+ 7 (- (- c14 c18) c23)) (- (- (- (- 254 (- (- 230 (+ (- c61 201) (+ c73 (- (- (+ c86 (- 239 123)) (- c106 (+ (- 195 (+ c124 (+ 0 c134))) (- 132 (- c152 248))))) (- c169 c174))))) c184)) (- (+ (+ c200 0) 132) 151)) (- (+ 28 (+ c231 (+ 18 (+ (+ (- 67 (+ 15 48)) c265) (+ (+ (+ c280 c285) c291) (- 22 (+ c306 (- (- 241 c321) 233)))))))) 100)) 1)))
"#);
}
#[test]
fn const_expr_251() {
    test_const_expr(r#"
const c12:u8=145
const c30:u8=0
static RESULT:u8
(= RESULT (- (- (+ (- c12 18) 128) (+ 0 c30)) 0))
"#);
}
#[test]
fn const_expr_252() {
    test_const_expr(r#"
const c30:u8=85
const c64:u8=250
const c72:u8=36
const c77:u8=61
const c85:u8=83
const c89:u8=65
const c98:u8=34
const c102:u8=70
const c113:u8=76
const c124:u8=0
const c139:u8=0
const c153:u8=99
const c165:u8=224
const c180:u8=0
const c197:u8=193
const c255:u8=83
const c260:u8=58
const c278:u8=109
const c288:u8=20
const c301:u8=247
const c306:u8=23
const c318:u8=41
const c326:u8=248
const c333:u8=29
const c339:u8=172
const c357:u8=0
const c378:u8=135
static RESULT:u8
(= RESULT (+ (- (- (- (+ (- (+ (- (- (+ c30 170) (+ (- 207 (+ (- (- (- (- c64 32) c72) c77) (- c85 c89)) (+ c98 c102))) (- c113 76))) c124) 0) (+ (+ c139 (+ (- (- c153 46) (- c165 171)) 0)) c180)) (- 123 (- c197 (- (- (+ (+ (+ (- (+ 116 117) (+ 33 (- (+ 37 188) (- c255 c260)))) (- (+ (- c278 (+ 5 c288)) (- (- c301 c306) (- 96 c318))) c326)) c333) c339) 12) 124)))) c357) 0) (+ (- (+ 44 c378) 179) 0)) 0))
"#);
}
#[test]
fn const_expr_253() {
    test_const_expr(r#"
const c18:u8=61
const c28:u8=1
const c63:u8=107
const c75:u8=21
const c87:u8=0
const c119:u8=251
const c124:u8=1
const c136:u8=243
const c141:u8=39
const c150:u8=3
const c155:u8=3
const c189:u8=0
const c233:u8=76
const c285:u8=0
const c300:u8=0
const c305:u8=2
const c311:u8=17
const c336:u8=176
const c350:u8=227
const c365:u8=29
const c370:u8=116
const c379:u8=113
const c411:u8=0
const c422:u8=0
const c430:u8=0
const c450:u8=29
const c460:u8=134
const c466:u8=44
const c474:u8=17
const c508:u8=13
const c516:u8=44
const c542:u8=43
const c549:u8=192
const c555:u8=0
static RESULT:u8
(= RESULT (+ (+ (- (+ (+ (- c18 (+ (+ c28 (- (- (- (+ 55 (- 217 (+ (+ (- c63 100) (+ c75 22)) (+ c87 (- 148 148))))) (- (+ (- (- c119 c124) (+ (- c136 c141) (- c150 c155))) 46) (- (+ 19 99) (+ (+ (+ c189 0) (- 180 174)) (- 227 (+ 27 162)))))) c233) 121)) 49)) 28) (- (- (+ 50 202) 11) (- (+ 223 c285) (+ (+ (+ c300 c305) c311) (- (- (+ (- 249 (- c336 (+ (- (- c350 68) (- (+ c365 c370) (- c379 99))) 144))) (+ (+ 0 (+ (+ c411 (+ (+ c422 0) c430)) 0)) (- (- (+ c450 149) c460) c466))) c474) (+ 24 145)))))) (- (+ 54 (+ c508 (- c516 (+ (- 242 239) 0)))) c542)) c549) c555))
"#);
}
#[test]
fn const_expr_254() {
    test_const_expr(r#"
const c63:u8=243
const c73:u8=196
const c86:u8=143
const c99:u8=75
const c110:u8=21
const c120:u8=2
static RESULT:u8
(= RESULT (+ (- (- (+ (- (- (- (- 248 (- 87 84)) (- 182 149)) (- 2 (- (- c63 (+ 39 c73)) (- 150 c86)))) 196) c99) (+ 21 c110)) (+ c120 4)) 213))
"#);
}
#[test]
fn const_expr_255() {
    test_const_expr(r#"
const c14:u8=14
const c22:u8=207
const c26:u8=143
const c32:u8=170
static RESULT:u8
(= RESULT (+ (+ (+ (+ 7 c14) (- c22 c26)) c32) 0))
"#);
}
#[test]
fn const_expr_256() {
    test_const_expr(r#"
const c6:u8=85
const c17:u8=37
static RESULT:u8
(= RESULT (- (+ c6 170) (- c17 37)))
"#);
}
