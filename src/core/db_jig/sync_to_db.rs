use crate::{
    core::state::AppState,
    domain::new_type_report::{
        error::NtrError,
        model::{NtrHead, RawNtrHead},
    },
};
use sqlx::{MySql, QueryBuilder};
use std::borrow::Cow;
use tracing::error;

pub async fn fetch_data(state: &AppState) -> Result<Vec<RawNtrHead>, NtrError> {
    let pool = state.pool();
    let datas: Vec<RawNtrHead> = sqlx::query_as::<_, RawNtrHead>(
        r#"
            SELECT 
                a.wo_part as 'item',
                a.wo_lot as 'wo_id',
                b.wod_part as 'comp'
            FROM dbqad_live.wo_mstr a 
            JOIN dbqad_live.wod_det b 
            ON a.wo_lot = b.wod_lot AND 
            SUBSTRING(b.wod_part, 1, 3) IN ('3CO','3CO','3SD','3DO','3DC','3CS','3YO','3TP','3MA')
            WHERE a.wo_nbr LIKE '%PROT%' 
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(datas)
}

pub async fn insert_into_database(
    state: &AppState,
    records: Vec<NtrHead>,
) -> Result<usize, NtrError> {
    if records.is_empty() {
        error!("!!>> No Data To Entry...");
        return Ok(0);
    }
    let pool = state.pool();
    let mut total_rows_affected = 0;
    for chunk in records.chunks(1000) {
        let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT IGNORE INTO db_wbd.nt_head(item, wo_id, comp, msr_type, ppr_type, create_date, created_by
            ) 
          "
        );
        query_builder.push_values(chunk, |mut b, rec| {
            b.push_bind(&rec.item)
                .push_bind(&rec.wo_id)
                .push_bind(&rec.comp)
                .push_bind(&rec.msr_type)
                .push_bind(&rec.ppr_type)
                .push_bind(state.clock.today())
                .push_bind(Cow::Borrowed("L04").as_ref());
        });

        let result = query_builder.build().execute(pool).await?;
        total_rows_affected += result.rows_affected();
    }
    let total_inserted = usize::try_from(total_rows_affected).unwrap_or(usize::MAX);
    Ok(total_inserted)
}
