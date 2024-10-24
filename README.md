# peppermint

A tiny query engine in rust, that illustrates how this works:

```sql
select count(distinct col1) from table1;
```

- a simplified logical planner with selection and projection
- a small sql parser
- naive/straw man counting distinct elements
- a morris counter
- a hyper log log
