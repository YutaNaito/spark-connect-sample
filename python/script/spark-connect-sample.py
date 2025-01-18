from pyspark.sql import SparkSession

SparkSession.builder.master("local[*]").getOrCreate().stop()

spark = SparkSession.builder.remote("sc://localhost:15002").getOrCreate()

data1 = [(1, "Alice"), (2, "Bob"), (3, "Charlie")]
columns1 = ["id", "name"]
data2 = [(1, "HR"), (2, "Engineering"), (4, "Marketing")]
columns2 = ["id", "department"]

df1 = spark.createDataFrame(data1, columns1)
df2 = spark.createDataFrame(data2, columns2)

joined_df = df1.join(df2, on="id", how="inner")

print("Joined DataFrame:")
joined_df.show()
joined_df.explain(True)

spark.stop()
