import collections.abc

from helpers import ascii_table

import sqlglot

short = "select 1 as a, case when 1 then 1 when 2 then 2 else 3 end as b, c from x"

def sqlglotrs_parse(sql):
    sqlglot.tokens.USE_RS_TOKENIZER = True
    sqlglot.parse_one(sql, error_level=sqlglot.ErrorLevel.IGNORE)

for i in range(1000):
    sqlglotrs_parse(short)