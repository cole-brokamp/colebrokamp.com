---
title: "Defining neighborhood geographies for clinical research"
description: "Census tracts, locally defined neighborhoods, and ZIP Code Tabulation Areas describe different places; choosing among them is a scientific decision, not a data-management shortcut."
date: "2025-11-04"
---

Clinical researchers often use a patient's neighborhood to represent social, economic, environmental, or policy context.
However, *neighborhood* is a meaningful human idea rather than a single national geography.
Census tracts, locally defined neighborhoods, ZIP codes, and ZIP Code Tabulation Areas describe different places and should not be treated as interchangeable.
The geography should follow the scientific question.
If the study concerns access to a city program, the relevant boundary may be the program's service area or a locally recognized neighborhood.
If the goal is to link standardized American Community Survey measures, a census tract may be the most defensible choice.
ZIP-derived geography may sometimes be necessary when more precise location data are unavailable, but it should be recognized as a postal proxy rather than assumed to represent a neighborhood.

## Neighborhood is a scientific construct before it is a polygon.

Before choosing a spatial file, define what *neighborhood* is expected to represent in the study.
Possibilities include a social community, an administrative jurisdiction, a policy environment, or the area around an individual's home.
Those concepts can imply different boundaries, and no polygon becomes correct merely because it is easy to obtain.

The choice also determines which people and places are averaged together.
A larger geography can conceal meaningful variation, while a smaller geography may produce unstable estimates or fail to represent the context through which an exposure operates.
Researchers should therefore document the geography type, boundary source, vintage, and reason for choosing it just as they would document a clinical variable's definition and measurement period.

## Census tracts are the strongest general-purpose starting point.

[Census tracts](https://www.census.gov/programs-surveys/geography/about/glossary.html#par_textimage_13) are small statistical subdivisions of counties designed to contain roughly 4,000 residents, generally ranging from 1,200 to 8,000.
They are relatively stable within a decennial period, have unique geographic identifiers, and are a common small-area geography for linking detailed American Community Survey estimates.

Tracts are still approximations rather than naturally occurring neighborhoods.
Their boundaries are revised after each decennial census as populations and local conditions change.
In Hamilton County, Ohio, there were 230 tracts in the 2000 vintage, 222 in 2010, and 226 in 2020.
Avondale was represented by five tracts in 2000 but four in 2010; those four remained the same in 2020.
An analysis must use a tract vintage that is consistent with the study period and the contextual data being linked.

The Census Bureau's [TIGER/Line files](https://www.census.gov/geographies/mapping-files/time-series/geo/tiger-line-file.html) provide the boundaries and identifiers used for this linkage.
TIGER/Line files contain geography, not the demographic estimates themselves; the geographic identifiers are what connect the boundaries to Census and American Community Survey tables.
The earlier [`cincy` geographies overview](https://geomarker.io/cincy/articles/geographies.html) maps these geography types across Hamilton County.

## Local neighborhood boundaries are best when local identity or policy matters.

Locally defined neighborhoods may better represent community identity, governance, service delivery, or intervention boundaries.
In Cincinnati, Community Council neighborhoods reflect locally recognized jurisdictions.
The city's [2020 Statistical Neighborhood Approximations dataset](https://data.cincinnati-oh.gov/dataset/Cincinnati-Statistical-Neighborhood-Approximations/i9zh-juvu) documents boundaries modified to fit the 2020 Census and 2016–2020 American Community Survey geographies so that demographic summaries can be calculated.

These boundary types serve related but different purposes.
A Community Council boundary may be most appropriate when the question concerns community identity or jurisdiction, while its tract-aligned approximation can support reproducible summaries of census data.
Because the two may share a name while covering slightly different areas, researchers should report which definition was used.

## ZIP codes should not be used as default neighborhood geographies.

A ZIP code is a postal routing label maintained by the U.S. Postal Service, not a polygon designed to represent population, environment, community identity, or policy context.
ZIP codes change as delivery operations change, and some identify individual buildings or organizations rather than areas.

A [ZIP Code Tabulation Area](https://www.census.gov/programs-surveys/geography/guidance/geo-areas/zctas.html) (ZCTA) is a Census Bureau approximation that turns commonly used ZIP codes into polygons for data tabulation.
ZCTAs make mapping possible, but they do not make postal geography scientifically equivalent to a neighborhood.
Their land area and population vary widely, and their boundaries follow the distribution of mailing addresses rather than a consistent social or contextual unit.

> ZIP code should almost never be the default neighborhood variable in a clinical study.

Using a ZIP code because it is already present in the medical record replaces a precise address with a coarse and often poorly aligned proxy.
That can mix patients who live in different communities, exclude patients who live in the community of interest, and distort the contextual characteristics assigned to the study population.

## Avondale shows how the ZIP code shortcut changes the population.

[Avondale](https://geomarker.io/cincy/articles/avondale.html) is a Cincinnati neighborhood with a strong local identity, but it does not align with postal geography.
The 45229 ZCTA is commonly used as a proxy for Avondale even though it includes North Avondale and excludes the western, eastern, and southern parts of Avondale.
The tract-aligned Avondale boundary intersects six ZCTAs, so no single ZIP-derived area represents it.
The boundary mismatch changes the resulting description of the population.
In the published example, tract-level American Community Survey measures summarized to Avondale were compared with the same measures summarized to ZCTA 45229.

[![Map of the tract-aligned Avondale neighborhood overlaid on six surrounding ZIP Code Tabulation Areas, beside a table comparing Avondale with ZCTA 45229: poverty is 43% versus 36%, median household income is $20,000 versus $31,000, and the Black non-Hispanic/Latino population is 78% versus 65%.](avondale-zcta-misalignment.svg)](avondale-zcta-misalignment.svg)

[Open the full-size figure.](avondale-zcta-misalignment.svg)

These differences show how the choice of boundary can affect the resulting neighborhood description.
In this example, using 45229 makes Avondale appear less economically deprived and changes the summarized racial composition.
A contextual exposure assigned using 45229 may therefore describe a somewhat different population and area than one based on the Avondale boundary.

## Geocode addresses and match them to the chosen geography.

When a study has residential addresses, the preferred workflow is to geocode each address within an appropriately governed computing environment and spatially match the resulting location to the prespecified tract, neighborhood, or other relevant boundary.
The analysis can retain only the derived geographic identifier when coordinates are not needed downstream, reducing unnecessary exposure of precise location data.

This approach separates two scientific decisions that ZIP-based workflows blur together: *where the patient lived* and *which contextual unit represents the hypothesized exposure*.
It also allows the same geocoded address to be linked reproducibly to more than one candidate geography for sensitivity analyses.

If only a ZIP code is available, use the corresponding ZCTA as a clearly labeled last-resort proxy, not as a synonym for neighborhood.
Describe the expected boundary mismatch, avoid claims about local community context that the postal geography cannot support, and assess a better-aligned geography in any subset with geocodable addresses.

## CoDEC can align Cincinnati data to the selected geography.

The [`codec` Cincinnati geography functions](https://geomarker.io/codec/reference/index.html#cincy-geographies) provide census tracts, neighborhoods, ZCTAs, cities, and address geographies as R spatial objects.
CoDEC tables can be read with `codec_read()` and interpolated on the fly by supplying a target from `cincy_neighborhood_geo()` or `cincy_zcta_geo()`.
This is useful when the available contextual data and the geography chosen for the study are not published at the same spatial resolution.

Interpolation is still an estimation step, not a way to erase a poor geographic definition.
It should be described along with the target geography and weighting choice.
See the short article on [spatial interpolation](https://geomarker.io/cincy/articles/interpolate.html) for details.

## A practical hierarchy keeps the geography tied to the science.

Use a locally defined neighborhood when community identity, jurisdiction, or a place-based intervention is the construct of interest.
Use census tracts when the study needs a standardized small-area geography for linking Census or American Community Survey measures.
Use another explicitly defined buffer, travel-time area, or service boundary when that better represents the hypothesized mechanism.

Do not begin with ZIP code merely because it is convenient.
If an address exists, use the address: geocode it, match it to the geography that represents the scientific construct, and document the boundary and vintage.
That small amount of additional spatial work prevents a postal routing system from silently defining the study's idea of neighborhood.
