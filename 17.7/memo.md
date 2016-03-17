# 17.7 HashMap

http://rustbyexample.com/std/hash.html

`parse<T: FromStr>(String) -> T` を用いた Scanner クラスを自作して使おうとしていたんだけど、
残念なことに `char` は `FromStr` trait を持ってない!

`Scanner::get_char`  を追加した

一応、入力に `ABC` が与えられた時点で `get_char` を呼ぶと、 `A` だけが消費されて `BC` は残すようにする.
つまり、空白で区切られて無くても `get_char` は一文字だけ呼ぶ.
ただし、それ以前にトークナイズはされてるので、空白文字や改行文字は消えてるので註意.

