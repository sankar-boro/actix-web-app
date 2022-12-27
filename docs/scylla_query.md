```
    let get_book_id = Uuid::parse_str(&book_id)?;
    let query_res = app.query(query, (&get_book_id,)).await?;
```