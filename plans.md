# まずはいろいろと数えていきましょう。

## セクション

Chap. 1 を除くと 22 章あるようです。


| Chapter no.   | Number of sections    |
| -             | -                     |
|1              | 6 sections            |
|2              | 7 sections            |
|3              | 7 sections            |
|4              | 8 sections            |

## 問題

#### 基準

- 基本的に、書きすぎる分には OK としています。
- 実装が絡まなそうなコラムは飛ばすかもです。
- 最後の「練習問題」は数だけを数えておきます。

| Chapter no.   | Number of exercises   |
| -             | -                     |
|2              | 56 exercises          |
|3              | 68 exercises          |
|4              | 57 exercises          |


#### 判例

- ⇔: ただのソースコード (digraph: `==`)
- 〇: 丸っぽい枠の問題 (digraph: `0_`)
- ☆: コラム (digraph: `*1`)

#### 統計

| Symbol        | Number    |
| -             | -         |
|〇             | 117       |
|⇔             | 27        |
|☆             | 14        |


#### 夏休みでやること

AGC の終わった翌日月曜日からカウントして、7 日間。
今のところなんの予定もなしですから、7 ×12 = 84 時間くらいは競プロをできるはず。
今年はバチャなどには目も触れず、あり本を進めていこうと思います。

2 章は流石に終わらせたいです。
〇 を数えると 33 問ありました。
後半厳しいですが、章は基本的に書いたことのあるものや
持っているものばかりですから、1 問平均 1 時間ちょっとくらいでいけると見込みたいです。


### 初級編

#### 2-1 全探索

- ⇔ 再帰関数
- ⇔ スタック
- 〇 部分和問題
- 〇 Lake Counting (POJ 2386)
- 〇 迷路の最短路
- ⇔ 特別な状態の列挙 (`next_permutation`)

#### 2-2 貪欲法

- 〇 硬貨の問題
- 〇 区間スケジューリング問題
- 〇 best Cow Line (POJ 3617) (辞書順最小の問題)
- 〇 Saruman's Army (POJ 3069)
- 〇 Fence Repair (POJ 3253)
- ☆ ハフマン符号


#### 2-3 動的計画法

- 〇 01ナップサック問題
- ☆ いろいろなDP
- 〇 最長共通部分列問題
- 〇 個数制限なしナップサック問題
- 〇 01ナップサック問題その 2
- 〇 個数制限付き部分和問題
- 〇 最長増加部分列問題
- ☆ `lower_bound`
- 〇 分割数
- 〇 重複組み合わせ


#### 2-4 データ構造

- ⇔ ヒープの実装例
- 〇 Expedition (POJ 2431)
- 〇 Fence Repair (PKU 3252) (貪欲と野菜じ問題）
- ⇔ 二分探索木の実装
- ☆ 平衡二分木
- ⇔ Union-Find木の実装
- 〇 食物連鎖 (POJ 1182)


#### 2-5 グラフ

- 〇 二部グラフ判定
- ⇔ ベルマンフォード法
- ⇔ ダイクストラ法
- ⇔ ワーシャル−フロイド法
- ⇔ 経路復元
- ⇔ プリム法
- ⇔ クラスカル法
- 〇 Roadblocks (POJ 3255)
- 〇 Conscription (POJ 3723)
- 〇 Layout (POJ 3169)

#### 2-6 数学的な問題を解くコツ

- 〇 線分上の格子点の個数
- 〇 双六
- 〇 素数判定
- 〇 素数の個数
- 〇 区間内の素数の個数
- 〇 Carmichael Numbers (UVa 100006)

#### 2-7 GCJの問題に挑戦してみよう(1)
- 〇 Minimum Scalar Product (2008 Round1A A)
- 〇 Crazy Rows (2009 Round2 A)
- 〇 Bribe the Prisoners
- 〇 Millionaire (2003 APAC local onsites C)

### 中級編

#### 3-1 二分探索

- 〇 `lower_bound`
- 〇 Cable master (POJ 1064)
- 〇 Aggressive cows (POJ 2456)
- 〇 平均最大化

#### 3-2 厳選！ 頻出テクニック(1)
- 〇 Subsequence (POJ 3061)
- 〇 Jessica's Reading Problem (POJ 3320)
- 〇 Face The Right Way (POJ 3276)
- 〇 Fliptile ( POJ 3279)
- ☆ 集合の整数表現
- 〇 Physics Experiment (POJ 3684)
- 〇 4 Values whose Sum is 0 (POJ 2785)
- 〇 巨大ナップザック
- 〇 座標圧縮

#### 3-3 さまざまなデータ構造を操ろう

- ⇔ セグメント木による RMQ の実装
- 〇 Crane (POJ 2991)
- ☆ Sparse Table による RMQ
- ⇔ BIT の実装（+ 二次元の BIT）
- 〇 バブルソートの交換回数
- 〇 A Simple Problem with integers (POJ 3468)
- 〇 K-th Number (2104)
- ☆ 領域木

#### 3-4 動的計画法を極める！

- 〇 巡回セールスマン問題
- 〇 Travelling by Stagegcoach (POJ 2686)
- 〇 ドミノ敷き詰め
- ☆ 完全マッチングの個数
- 〇 フィボナッチ数列
- ☆ もっと高速な漸化式の計算
- 〇 Blocks (POJ 3734)
- 〇 グラフの長さkのパスの総数
- 〇 Matrix Power Series (POJ 3233)
- 〇 Minimizing maximizer (POJ 1769)

#### 3-5 ネットワークフロー

- 〇 最大通信量
- ☆ さまざまなグラフに対する最大流
- 〇 高速な最大流アルゴリズム
- 〇 仕事の割り当て
- 〇 二人組
- 〇 最小コスト通信
- ☆ さまざまなグラフに対する最小費用流
- 〇 Asteroids (POJ 3041)
- 〇 Evacuation (POJ 3057)
- 〇 Dining (POJ 3281)
- 〇 Dual Core CPU (POJ 3269)
- 〇 Farm Tour (POJ 2135)
- 〇 Evacuation Plan (POJ 2175)
- 〇 The Windy's (POJ 3286)
- 〇 Intervals (POJ 3680)
- ☆ 線形計画問題

#### 3-6 計算幾何

- 〇 Jack Straws (POJ 1127)
- 〇 White Bird (AOJ 2308)
- 〇 Coneology (POJ 2932)
- 〇 Beauty Contest (POJ 2187)
- 〇 Intersection of Two Primms (AOJ 1313)

#### 3-7 GCJ の問題に挑戦してみよう(2)

- 〇 Numbers (2008 Round 1A C)
- 〇 No Cheating (2008 Round 3 C)
- 〇 Stock Charts (2009 Round 2 C)
- 〇 Watering Planets (2009 Round 2 D)
- 〇 Number Sets (2008 Round 1B B)
- 〇 Wi-fi Towers (2009 World Final D)

### 上級編

#### 4-1 より複雑な数学的問題

- ⇔ 連立一次方程式
- 〇 ランダムウォーク
- ⇔ 逆元
- ⇔ フェルマーの小定理
- ⇔ 連立線形合同式
- ⇔ 中国剰余定理
- ⇔ n!
- ⇔ 二項係数
- 〇 包除原理
- 〇 メビウス関数
- 〇 石の塗り方の数え上げ


#### 4-2 ゲームの必勝法を編み出せ!

- 〇 コインのゲーム1
- 〇 A Funny Game (POJ 2484)
- 〇 Euclid's Game (POJ 2348)
- 〇 Nim
- 〇 Georgia and Bob (POJ 1704)
- 〇 コインのゲーム2
- 〇 Cutting Game (POJ 2311)


#### 4-3 グラフマスターへの道

- ⇔ 強連結成分分解
- 〇 Popular Cows (POJ 2186)
- ⇔ 2-SAT
- 〇 Priest John's Buiest Day (POJ 3683)
- ⇔ LCA 二分探索を用いる方法
- ⇔ LCA RMQ を用いる手法
- 〇 Housewife Wind (POJ 2763)


#### 4-4 厳選！ 頻出テクニック(2)

- 〇 Largest Rectangle in a Histogram (POJ 2559)
- 〇 スライド最小値
- 〇 個数成約付きナップザック
- 〇 K-Anonymous Sequence (POJ 3709)
- 〇 巡回スケジューリング


#### 4-5 工夫を凝らして賢く探索

- 〇 数独 (POJ 2676, 2918, 3074, 3076)
- 〇 Square Destroyer (PKU 1084)
- ⇔ IDA\*
- ⇔ AD\*
- ☆ 整数計画問題

#### 4-6 分割統治法

- 〇 バブルソートの交換回数
- 〇 Tree (POJ 1741)
- 〇 最近点対問題 (UVa 10245)


#### 4-7 文字列を華麗に扱う

- 〇 禁止文字列
- 〇 DNA Repair (POJ 3691)
- ☆ Trie
- 〇 文字列 DP のより高速な前処理
- 〇 星座 (POJ 3690)
- 〇 Sequence (POJ 3581)
- 〇 共通部分文字列 (POJ 2217)
- 〇 最長回文

### 4-8 GCJ の問題に挑戦してみよう

- 〇 Mine Layer (2008 World Final C)
- 〇 Year of More Code Jam (2009 World Final A)
- 〇 Football Team (2009 Round 3 C)
- 〇 Endless Knight (2008 Round 3 D)
- 〇 Year of Code Jam (2008 World Final E)
