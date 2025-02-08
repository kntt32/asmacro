# asmacro
## 概要
asmacroは、x64アセンブリをマクロの力で、高性能なまま書きやすくすることを目指しているプロジェクトである  
Rustによるブートストラップ用のものとasmacro自身による、二種類のアセンブラ/プリプロセッサにより構成される予定である  
「マクロでがんばるコンパイラ」でもある。

## 現在地
Rustによるブートストラップ用のアセンブラがほぼほぼ完成した状態である。  
設計ミスや機械語生成に関する勘違いで、ここまで時間がかかってしまった。これから実装予定の部分は、比較的はやく作成可能だと思われる。

## とりあえずの目標
↓このようなコードを、
```
fn main() : (edx, esi, eax, fibo, aaa, stdout) -> () {
    let mut a: u32@edx = 1;
    let mut b: u32@esi = 1;
  
    loop 100 {
        let c: u32@eax = fibo(a@edi, b);
        a = b;
        b = c;
        stdout(aaa(c@edi));
    }
}

fn fibo(a: u32@rdi, b: u32@esi) : () -> u32@eax {
    a@rax + b
}
```
↓このようなアセンブリに変換し、
```
main:
    mov rdi 1
    mov rsi 1
    mov rcx 100
main_loop_begin:
    call fibo
    mov edx eax
    mov esi eax
    mov edi eax
    call aaa
    call stdout
    loop main_loop_begin
    ret
    
fibo:
    mov edi, eax
    add eax, esi
    ret
```
さらにELF形式に自力で変換することだ

## 参考文献
- AMD64仕様書 https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24594.pdf
- Oracleのマニュアル https://docs.oracle.com/cd/E19683-01/817-4912/6mkdg541g/index.html
- 最小限のELF https://keens.github.io/blog/2020/04/12/saishougennoelf/
