use crate::uniV3PoolGetter;

pub async fn filter(file: &str) -> std::io::Result<Vec<Pool>> {
    let pairs = uniV3PoolGetter::get_pools_list(file).await?;

    //
    //
}
