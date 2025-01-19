# spark-connect-sample

## version
- Apache Spark 4.0.0-preview1
- Python3 3.12

## Run

### Spark Connect Server
- Start Spark Connect Server (default: standalone mode)
```shell
$ start-connect-server.sh \
  --packages org.apache.spark:spark-connect_2.13:4.0.0-preview1
```

- Start Spark Connect Server using loccal cluster
```shell
$ start-connect-server.sh \
  --packages org.apache.spark:spark-connect_2.13:4.0.0-preview1 \
  --master local-cluster[2,2,2042]
```

### python client
```shell
$ python3 spark-connect.py
~
~
Joined DataFrame:
+---+-----+-----------+
| id| name| department|
+---+-----+-----------+
|  1|Alice|         HR|
|  2|  Bob|Engineering|
+---+-----+-----------+

== Parsed Logical Plan ==
'Join UsingJoin(Inner, [id])
:- Project [id#516L AS id#520L, name#517 AS name#521]
:  +- LocalRelation [id#516L, name#517]
+- Project [id#536L AS id#540L, department#537 AS department#541]
   +- LocalRelation [id#536L, department#537]

== Analyzed Logical Plan ==
id: bigint, name: string, department: string
Project [id#520L, name#521, department#541]
+- Join Inner, (id#520L = id#540L)
   :- Project [id#516L AS id#520L, name#517 AS name#521]
   :  +- LocalRelation [id#516L, name#517]
   +- Project [id#536L AS id#540L, department#537 AS department#541]
      +- LocalRelation [id#536L, department#537]

== Optimized Logical Plan ==
Project [id#520L, name#521, department#541]
+- Join Inner, (id#520L = id#540L)
   :- LocalRelation [id#520L, name#521]
   +- LocalRelation [id#540L, department#541]

== Physical Plan ==
AdaptiveSparkPlan isFinalPlan=false
+- Project [id#520L, name#521, department#541]
   +- BroadcastHashJoin [id#520L], [id#540L], Inner, BuildRight, false
      :- LocalTableScan [id#520L, name#521]
      +- BroadcastExchange HashedRelationBroadcastMode(List(input[0, bigint, true]),false), [plan_id=629]
         +- LocalTableScan [id#540L, department#541]
```

### rust client
```shell
$ cargo run
~
~
+--------------------------+
| show_string              |
+--------------------------+
| +---+-------+----------+ |
| | id|   name|department| |
| +---+-------+----------+ |
| |  1|  Alice|        HR| |
| |  2|    Bob|Enginee...| |
| |  3|Charlie| Marketing| |
| +---+-------+----------+ |
|                          |
+--------------------------+
== Parsed Logical Plan ==
'Project ['id, 'name, 'department]
+- 'Join Inner, '`==`('id, 'id)
   :- LocalRelation [id#437L, name#438]
   +- LocalRelation [id#439L, department#440]

== Analyzed Logical Plan ==
id: bigint, name: string, department: string
Project [id#437L, name#438, department#440]
+- Join Inner, (id#437L = id#439L)
   :- LocalRelation [id#437L, name#438]
   +- LocalRelation [id#439L, department#440]

== Optimized Logical Plan ==
Project [id#437L, name#438, department#440]
+- Join Inner, (id#437L = id#439L)
   :- LocalRelation [id#437L, name#438]
   +- LocalRelation [id#439L, department#440]

== Physical Plan ==
AdaptiveSparkPlan isFinalPlan=false
+- Project [id#437L, name#438, department#440]
   +- BroadcastHashJoin [id#437L], [id#439L], Inner, BuildRight, false
      :- LocalTableScan [id#437L, name#438]
      +- BroadcastExchange HashedRelationBroadcastMode(List(input[0, bigint, false]),false), [plan_id=569]
         +- LocalTableScan [id#439L, department#440]
```
