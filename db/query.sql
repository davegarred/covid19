select * from records;

truncate records;

SELECT r.state,
       r.date,
       r.value,
       r2.date                                                     prev_day,
       r2.value                                                    prev_val,
       pow(cast(r.value as float) / cast(r2.value as float), .333) - 1 delta
FROM records r
         INNER JOIN (SELECT country, state, value, date, (to_date(date, 'MM/DD/YY') + 3) AS true_date FROM records) r2
                    ON (
                            to_date(r.date, 'MM/DD/YY') = r2.true_date AND
                            r.country = r2.country AND
                            r.state = r2.state
                        )
WHERE (
--         r.state = 'King County, WA' OR r.state = 'Washington'
--         r.country = 'Korea, South'
        r.country = 'Italy'
    )
  AND r.record_type = 'CONFIRMED'
ORDER BY to_date(r.date, 'MM/DD/YY')
;

SELECT 'all',
       r.date,
       sum(r.value),
       r2.date                                                     prev_day,
       sum(r2.value)                                                    prev_val,
       pow(cast(sum(r.value) as float) / cast(sum(r2.value) as float), .333) - 1 delta
FROM records r
         INNER JOIN (SELECT country, state, value, date, (to_date(date, 'MM/DD/YY') + 3) AS true_date FROM records) r2
                    ON (
                            to_date(r.date, 'MM/DD/YY') = r2.true_date AND
                            r.country = r2.country AND
                            r.state = r2.state
                        )
WHERE (
        -- r.state = 'King County, WA' OR
        -- r.state = 'Washington'
        r.country = 'US'
    )
  AND r.record_type = 'CONFIRMED'
GROUP BY r.date, r2.date
ORDER BY to_date(r.date, 'MM/DD/YY')
;
select country from records group by country order by country;
