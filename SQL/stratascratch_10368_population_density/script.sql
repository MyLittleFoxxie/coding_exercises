-- You are working on a data analysis project at Deloitte where you need to analyze a dataset containing information
-- about various cities. Your task is to calculate the population density of these cities, rounded to the nearest integer, and identify the cities with the minimum and maximum densities.
-- The population density should be calculated as (Population / Area).


-- The output should contain 'city', 'country', 'density'.

(
    SELECT
        city,
        country,
        ROUND(population / area) AS density
    FROM
        cities_population
    ORDER BY
        density DESC
    LIMIT 1
)
UNION ALL
(
    SELECT
        city,
        country,
        ROUND(population / area) AS density
    FROM
        cities_population
    ORDER BY
        density ASC
    LIMIT 1
);
