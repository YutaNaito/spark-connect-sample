use arrow_array::{ArrayRef, Int64Array, RecordBatch, StringArray};
use spark_connect_rs::dataframe::{ExplainMode, JoinType};
use spark_connect_rs::functions::col;
use spark_connect_rs::{SparkSession, SparkSessionBuilder};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spark: SparkSession = SparkSessionBuilder::remote("sc://127.0.0.1:15002/;user_id=YN")
        .build()
        .await?;

    let id: ArrayRef = Arc::new(Int64Array::from(vec![1, 2, 3]));
    let name: ArrayRef = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie"]));
    let department: ArrayRef = Arc::new(StringArray::from(vec!["HR", "Engineering", "Marketing"]));

    let data1 = RecordBatch::try_from_iter(vec![("id", id.clone()), ("name", name.clone())])?;
    let data2 =
        RecordBatch::try_from_iter(vec![("number", id.clone()), ("department", department.clone())])?;

    let df1 = spark.create_dataframe(&data1)?;
    let df2 = spark.create_dataframe(&data2)?;

    let condition = Some(col("id").eq(col("number")));
    let joined_df = df1.clone().join(df2, condition, JoinType::Inner).select([
        col("id"),
        col("name"),
        col("department"),
    ]);
    joined_df
        .clone()
        .show(Some(10), Some(10), Some(false))
        .await?;
    joined_df.explain(Some(ExplainMode::Extended)).await?;

    Ok(())
}
