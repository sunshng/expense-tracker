use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

/// 账单记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    #[serde(rename = "type")]
    pub trans_type: String,
    pub amount: f64,
    pub category_l1: String,
    pub category_l2: String,
    pub date: String,
    pub note: String,
    pub created_at: String,
}

/// 分类
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: String,
    pub sort_order: i64,
}

/// 添加账单的请求参数
#[derive(Debug, Deserialize)]
pub struct AddTransactionRequest {
    #[serde(rename = "type")]
    pub trans_type: String,
    pub amount: f64,
    pub category_l1: String,
    pub category_l2: String,
    pub date: String,
    pub note: Option<String>,
}

/// 更新账单的请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateTransactionRequest {
    #[serde(rename = "type")]
    pub trans_type: String,
    pub amount: f64,
    pub category_l1: String,
    pub category_l2: String,
    pub date: String,
    pub note: Option<String>,
}

/// 筛选条件
#[derive(Debug, Deserialize)]
pub struct FilterParams {
    #[serde(rename = "type")]
    pub trans_type: Option<String>,
    pub category_l1: Option<String>,
    pub category_l2: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub keyword: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// 统计结果
#[derive(Debug, Serialize)]
pub struct Statistics {
    #[serde(rename = "byCategory")]
    pub by_category: Vec<CategoryStats>,
    pub monthly: Vec<MonthlyStats>,
    pub summary: SummaryStats,
}

#[derive(Debug, Serialize)]
pub struct CategoryStats {
    pub category_l1: String,
    pub total: f64,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct MonthlyStats {
    pub month: String,
    pub total_expense: f64,
    pub total_income: f64,
}

#[derive(Debug, Serialize)]
pub struct SummaryStats {
    pub total_expense: f64,
    pub total_income: f64,
}

/// 数据库管理器
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// 初始化数据库：创建表并插入默认分类
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

        // 开启 WAL 模式
        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .map_err(|e| e.to_string())?;

        // 创建表
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type TEXT NOT NULL CHECK(type IN ('expense', 'income')),
                amount REAL NOT NULL,
                category_l1 TEXT NOT NULL,
                category_l2 TEXT NOT NULL,
                date TEXT NOT NULL,
                note TEXT DEFAULT '',
                created_at TEXT DEFAULT (datetime('now', 'localtime'))
            );

            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                parent_id INTEGER DEFAULT NULL,
                icon TEXT DEFAULT '',
                sort_order INTEGER DEFAULT 0,
                FOREIGN KEY (parent_id) REFERENCES categories(id)
            );"
        ).map_err(|e| e.to_string())?;

        // 插入默认分类
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        if count == 0 {
            Self::insert_default_categories(&conn)?;
        }

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    /// 插入默认两级分类
    fn insert_default_categories(conn: &Connection) -> Result<(), String> {
        let categories = vec![
            ("餐饮", "🍽️", vec!["早餐", "午餐", "晚餐", "零食饮料", "聚餐请客", "外卖"]),
            ("交通", "🚗", vec!["公交地铁", "出租车/网约车", "高铁/火车", "飞机", "加油", "停车费"]),
            ("购物", "🛒", vec!["日用品", "数码产品", "家居用品", "宠物用品", "办公用品"]),
            ("住房", "🏠", vec!["房租", "水电费", "燃气费", "物业费", "维修", "网费"]),
            ("娱乐", "🎮", vec!["电影", "游戏", "旅游", "运动健身", "KTV/酒吧", "书籍"]),
            ("医疗", "🏥", vec!["门诊", "药品", "体检", "住院", "牙科"]),
            ("教育", "📚", vec!["培训课程", "书本教材", "考试报名", "文具"]),
            ("通讯", "📱", vec!["话费", "快递"]),
            ("服饰", "👗", vec!["衣服", "鞋子", "包包", "饰品", "化妆品"]),
            ("亲子", "👶", vec!["奶粉", "尿布", "玩具", "教育"]),
            ("人情", "🎁", vec!["送礼", "红包", "捐款", "婚礼份子"]),
            ("收入", "💰", vec!["工资", "奖金", "兼职", "投资收益", "红包收入", "退款", "其他收入"]),
            ("其他", "🔧", vec!["不确定分类"]),
        ];

        for (idx, (name, icon, children)) in categories.iter().enumerate() {
            conn.execute(
                "INSERT INTO categories (name, parent_id, icon, sort_order) VALUES (?1, NULL, ?2, ?3)",
                params![name, icon, idx as i64],
            ).map_err(|e| e.to_string())?;

            let parent_id: i64 = conn.query_row(
                "SELECT last_insert_rowid()", [], |row| row.get(0)
            ).map_err(|e| e.to_string())?;

            for (child_idx, child_name) in children.iter().enumerate() {
                conn.execute(
                    "INSERT INTO categories (name, parent_id, icon, sort_order) VALUES (?1, ?2, '', ?3)",
                    params![child_name, parent_id, child_idx as i64],
                ).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    /// 获取分类列表
    pub fn get_categories(&self) -> Result<Vec<Category>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT * FROM categories ORDER BY sort_order")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_id: row.get(2)?,
                    icon: row.get(3)?,
                    sort_order: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut categories = Vec::new();
        for row in rows {
            categories.push(row.map_err(|e| e.to_string())?);
        }
        Ok(categories)
    }

    /// 获取账单列表
    pub fn get_transactions(&self, filters: FilterParams) -> Result<Vec<Transaction>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut sql = String::from("SELECT * FROM transactions WHERE 1=1");
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref t) = filters.trans_type {
            sql.push_str(" AND type = ?");
            param_values.push(Box::new(t.clone()));
        }
        if let Some(ref cat) = filters.category_l1 {
            sql.push_str(" AND category_l1 = ?");
            param_values.push(Box::new(cat.clone()));
        }
        if let Some(ref cat) = filters.category_l2 {
            sql.push_str(" AND category_l2 = ?");
            param_values.push(Box::new(cat.clone()));
        }
        if let Some(ref start) = filters.start_date {
            sql.push_str(" AND date >= ?");
            param_values.push(Box::new(start.clone()));
        }
        if let Some(ref end) = filters.end_date {
            sql.push_str(" AND date <= ?");
            param_values.push(Box::new(end.clone()));
        }
        if let Some(ref kw) = filters.keyword {
            sql.push_str(" AND note LIKE ?");
            param_values.push(Box::new(format!("%{}%", kw)));
        }

        sql.push_str(" ORDER BY date DESC, id DESC");

        if let Some(limit) = filters.limit {
            sql.push_str(" LIMIT ?");
            param_values.push(Box::new(limit));
        }
        if let Some(offset) = filters.offset {
            sql.push_str(" OFFSET ?");
            param_values.push(Box::new(offset));
        }

        // 将参数转为引用切片
        let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(Transaction {
                    id: row.get(0)?,
                    trans_type: row.get(1)?,
                    amount: row.get(2)?,
                    category_l1: row.get(3)?,
                    category_l2: row.get(4)?,
                    date: row.get(5)?,
                    note: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(row.map_err(|e| e.to_string())?);
        }
        Ok(transactions)
    }

    /// 添加账单
    pub fn add_transaction(&self, req: AddTransactionRequest) -> Result<Transaction, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let note = req.note.unwrap_or_default();
        conn.execute(
            "INSERT INTO transactions (type, amount, category_l1, category_l2, date, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![req.trans_type, req.amount, req.category_l1, req.category_l2, req.date, note],
        ).map_err(|e| e.to_string())?;

        let id = conn.last_insert_rowid();
        Ok(Transaction {
            id,
            trans_type: req.trans_type,
            amount: req.amount,
            category_l1: req.category_l1,
            category_l2: req.category_l2,
            date: req.date,
            note,
            created_at: String::new(),
        })
    }

    /// 删除账单
    pub fn delete_transaction(&self, id: i64) -> Result<bool, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let affected = conn
            .execute("DELETE FROM transactions WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(affected > 0)
    }

    /// 更新账单
    pub fn update_transaction(&self, id: i64, req: UpdateTransactionRequest) -> Result<bool, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let note = req.note.unwrap_or_default();
        let affected = conn
            .execute(
                "UPDATE transactions SET type=?1, amount=?2, category_l1=?3, category_l2=?4, date=?5, note=?6 WHERE id=?7",
                params![req.trans_type, req.amount, req.category_l1, req.category_l2, req.date, note, id],
            )
            .map_err(|e| e.to_string())?;
        Ok(affected > 0)
    }

    /// 获取统计数据
    pub fn get_statistics(&self, start_date: Option<String>, end_date: Option<String>) -> Result<Statistics, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut where_clause = String::new();
        let mut params_list: Vec<String> = Vec::new();

        if let Some(ref start) = start_date {
            where_clause.push_str(" AND date >= ?");
            params_list.push(start.clone());
        }
        if let Some(ref end) = end_date {
            where_clause.push_str(" AND date <= ?");
            params_list.push(end.clone());
        }

        // 按分类统计
        let cat_sql = format!(
            "SELECT category_l1, SUM(amount) as total, COUNT(*) as count
             FROM transactions WHERE type = 'expense' {}
             GROUP BY category_l1 ORDER BY total DESC",
            where_clause
        );

        let mut cat_stmt = conn.prepare(&cat_sql).map_err(|e| e.to_string())?;
        let cat_params: Vec<&dyn rusqlite::types::ToSql> = params_list.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();
        let cat_rows = cat_stmt
            .query_map(cat_params.as_slice(), |row| {
                Ok(CategoryStats {
                    category_l1: row.get(0)?,
                    total: row.get(1)?,
                    count: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut by_category = Vec::new();
        for row in cat_rows {
            by_category.push(row.map_err(|e| e.to_string())?);
        }

        // 按月统计
        let monthly_sql = format!(
            "SELECT substr(date, 1, 7) as month,
                    SUM(CASE WHEN type='expense' THEN amount ELSE 0 END) as total_expense,
                    SUM(CASE WHEN type='income' THEN amount ELSE 0 END) as total_income
             FROM transactions WHERE 1=1 {}
             GROUP BY month ORDER BY month ASC LIMIT 12",
            where_clause
        );

        let mut mon_stmt = conn.prepare(&monthly_sql).map_err(|e| e.to_string())?;
        let mon_params: Vec<&dyn rusqlite::types::ToSql> = params_list.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();
        let mon_rows = mon_stmt
            .query_map(mon_params.as_slice(), |row| {
                Ok(MonthlyStats {
                    month: row.get(0)?,
                    total_expense: row.get(1)?,
                    total_income: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut monthly = Vec::new();
        for row in mon_rows {
            monthly.push(row.map_err(|e| e.to_string())?);
        }

        // 总收支
        let summary_sql = format!(
            "SELECT COALESCE(SUM(CASE WHEN type='expense' THEN amount ELSE 0 END), 0) as total_expense,
                    COALESCE(SUM(CASE WHEN type='income' THEN amount ELSE 0 END), 0) as total_income
             FROM transactions WHERE 1=1 {}",
            where_clause
        );

        let mut sum_stmt = conn.prepare(&summary_sql).map_err(|e| e.to_string())?;
        let sum_params: Vec<&dyn rusqlite::types::ToSql> = params_list.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();
        let summary = sum_stmt
            .query_row(sum_params.as_slice(), |row| {
                Ok(SummaryStats {
                    total_expense: row.get(0)?,
                    total_income: row.get(1)?,
                })
            })
            .map_err(|e| e.to_string())?;

        Ok(Statistics {
            by_category,
            monthly,
            summary,
        })
    }
}
