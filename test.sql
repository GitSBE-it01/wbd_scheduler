WITH 
demand_data  AS (
    SELECT 
        dm_a.wod_part as 'item', 
        dm_a.wod_lot AS 'lot__id',
        dm_b.wo__chr04 AS 'loc__line',
        dm_c.dept as 'dept',
        (dm_a.wod_qty_req - dm_a.wod_qty_iss) as 'qty',
        dm_b.wo_rel_datex as '_date',
        dm_b.wo_rel_datex as 'rel_dt',
        dm_b.wo_due_datex as 'due_dt',
        dm_b.wo_rmks as 'wo_rmks',
        dm_b.wo_part as 'item_id',
        dm_b.wo_vend as 'old_id'
      FROM dbqad_live.wod_det dm_a 
      JOIN dbqad_live.wo_mstr dm_b  
        ON dm_a.wod_lot = dm_b.wo_lot
      JOIN dbpick_now.dept_new dm_c
      ON dm_b.wo__chr04 = dm_c.assyLine
      WHERE dm_b.wo_status = 'R' AND dm_b.wo__chr04 != '' 
      ORDER BY dm_b.wo_vend, dm_a.wod_lot, dm_a.wod_part
),
inventory_data AS (
      SELECT 
        inv_a.ld_part AS 'item', 
        CASE 
              WHEN inv_b.loc_department = 'PROD' THEN 'P1.ASSY'
              WHEN inv_b.loc_loc = 'WHPR' THEN 'WH_ASSY'
              ELSE inv_b.loc_department
              END
        AS 'dept',
        '2000-01-01' as '_date',
        SUM(inv_a.ld_qty_oh) AS 'qty_OH',
        GROUP_CONCAT(
          CONCAT(
              inv_a.ld_lot, '-', inv_a.ld_loc, '=', 
              IF(inv_a.ld_qty_oh LIKE '.%', CONCAT('0', inv_a.ld_qty_oh), CAST(inv_a.ld_qty_oh AS CHAR))
          ) 
          SEPARATOR '--')
        AS 'lotOH'
      FROM dbqad_live.ld_det inv_a 
      JOIN dbqad_live.loc_mstr inv_b  
        ON inv_a.ld_loc = inv_b.loc_loc
      WHERE inv_a.ld_status NOT IN ('AVALAN', 'WKNDA', 'UNCOUNT', 'TRANSIT', 'CONSIGN', 'SCRAP', 'REJECT', 'SAMPLE')  
      GROUP BY inv_a.ld_part, inv_b.loc_department
      ORDER BY inv_a.ld_part ASC, inv_b.loc_department
),
demand_with_running_sum AS (
    SELECT 
        CURDATE() as 'data_date',
        dmd.*,
        pic.optr,
        COALESCE(i.available_stock, 0) AS total_inventory_qty,
        SUM(d.qty_needed) OVER (
            PARTITION BY d.division_id, d.item_id 
            ORDER BY d.need_date ASC, d.wo_id ASC
        ) AS cumulative_demand
    FROM demand_data dmd
    LEFT JOIN inventory_data inv
      ON d.division_id = i.division_id 
      AND d.item_id = i.item_id
    LEFT JOIN dbpick_now.pic_part_category  pic
        ON dmd.item = pic.tipe
    ORDER BY dmd.item ASC, dmd.loc_department, dmd._date
)