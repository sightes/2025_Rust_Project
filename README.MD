Allowed languages: `Rust`, `sql` & `bash`.

Let's create a new project:

```bash
# Create your project structure
make analysis=spot_segmentation

# Delete project structure (remember to commit your progress)
make clean analyis=spot_segmentation
```

**Directory Structure**

* snake_case https://en.wikipedia.org/wiki/Snake_case e.g. The project title is Spot Segmentation, the folder should be spot_segmentation
* date analysis: this kind of analysis are time related, you're not expected to work more on this after the release of the results YYYY_MM_DD_<project_name> e.g. 2024_04_03_exploratory_supply.

> Practical usage: [./demo/explore.ipynb](./demo/explore.ipynb)

Rationale about this rules: https://medium.com/@joshuale/a-practical-guide-to-python-project-structure-and-packaging-90c7f7a04f95

Expected folder structure:
```bash
/<analysis>
    /sql
        <query>.sql
        ...
    __iniy__.py
    main.py
    explore.ipynb
    README.md
```

**How to read SQL files?**

* [https://stackoverflow.com/a/19473206/8567062](https://stackoverflow.com/a/19473206/8567062)
* https://docs.sqlfluff.com/en/stable/configuration.html#jinja-templater


**How to format SQL:**

https://docs.sqlfluff.com/en/stable/gettingstarted.html

```bash
sqlfluff lint ./path/sql/*.sql --dialect mysql
sqlfluff fix ./path/sql/*.sql --force # Can help to fix the SQL File
```

2. Query Evaluation: All queries must be evaluable to determine if they will serve as the source of truth.

> * Use https://metabase.spot2.mx
>
> * Use direct connection to database using python and formatting with jinja2

3. Metric Idempotence: Metrics must be idempotent over equal time periods; they should yield the same numbers. This requires a consistent query and data source, both serving as the source of truth. Metrics are reproducible.

> https://platform.spot2.mx is the source of truth if you need to understand a query use Telescope, docs  [here](https://spot2mx.atlassian.net/wiki/spaces/SDKB/pages/90177568/Sobre+Debugging).

4. Data Transformation Documentation: Transformations applied to data must be documented.

> Using panda or polars? Document your lambdas or pipelines.

5. Primary Keys: The primary keys to use are the table IDs and email addresses to identify individuals.

```sql
SELECT id FROM users;
SELECT id FROM spots;
...
```

6. Data Privacy: Do not expose sensitive personal data such as DATABASE CREDENTIALS, names, numbers, job positions, addresses, photographs, etc.

> Practical usage of .env [./demo/explore.ipynb](./demo/explore.ipynb)

7. Graphical Explanations: While graphics can speak for themselves, it is beneficial to explain how they were derived.

> Library for graphics:
>
> * Static: https://seaborn.pydata.org/ TODO: Guidelines for graphs
> * Dynamic: https://recharts.org/en-US/

8. Metric Evaluation: Metrics should be evaluated from both sides using LEFT JOIN and RIGHT JOIN.

> Find and report gaps in data.

9. Query Optimization: Queries should be optimized once they meet the business need.


10.  Data Storage: Information should be stored in readable formats and encoded in UTF-8, such as CSV, XLSX, JSON, PARQUET, and SQL files.

> This repository ignores all this kind of files, those are only final files.
