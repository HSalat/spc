# Data sources

The data is sorted around the 2011 Middle-layer Super Output Area (MSOA) geographical unit. These units where created for census collection and are designed to be relatively homogeneous, with an average population size of 8000. Any list of MSOAs in England can be run, with the exception of the MSOAs forming the City of London (i.e the London borough called the City, not London as a whole).

The data from Open Street Map (OSM) is downloaded directly from https://www.openstreetmap.org. Everything else is hosted as local copies on one Azure repository that interacts automatically with the model, and divided into utilities, county level data and national data.

## Utility data

```
lookUp.csv
```

The look-up table links different geographies together. It is used internally by the model, but can also help the user define their own study area. `MSOA11CD`, `MSOA11NM`, `LAD20CD`, `LAD20NM`, `ITL321CD`, `ITL321NM`, `ITL221CD`, `ITL221NM`, `ITL121CD`, `ITL121NM` are all standard denominations fully compatible with ONS fields of the same name. They are based on ONS [lookups](https://geoportal.statistics.gov.uk/). See ONS documentation for more details. `CTY20NM` and `CCTY20NM` are custom denominations for the counties of England (used to sort the county level population data) and the ceremonial counties of England respectively. Their spelling may vary in different data sources and the field `CTY20NM` is not compatible with the ONS field of the same name (which excludes all counties that are also unitary authorities). `GoogleMob` and `OSM` are different spellings for the counties of England used by Google and OSM for their data releases.

## County level data

Contains 47 files, each representing one of the counties of England mentioned above and named

```
tus_hse_<county_name>.gz
```

This data is based on the [2011 UK census](http://dx.doi.org/10.5257/census/aggregate-2011-1), the [Time Use Survey 2014-15](http://dx.doi.org/10.5255/UKDA-SN-8128-1) and the [Health Survey for England 2017](http://dx.doi.org/10.5255/UKDA-SN-8488-2). The SPENSER (Synthetic Population Estimation and Scenario Projection) microsimulation model ([reference](http://dx.doi.org/10.1111/gean.12320)) distributes a synthetic population based on the census at MSOA scale and projects it to 2020 according to estimates from the Office for National Statistics (ONS). This information was enriched with some of the content of the other two datasets through propensity score matching (PSM) by Karyn Morrissey (Technical University of Denmark). The rest of the datasets can be added *a posteriori* from the identifiers provided.

The fields currently contained are:
- `idp`: a unique individual identifier within the present data
- `MSOA11CD`: MSOA code where the individual lives
- `hid`: household identifier, includes communal establishments
- `pid`: identifier linking to the 2011 Census
- `pid_tus`: identifier linking to the Time Use Survey 2015
- `pid_hse`: identifier linking to the Health Survey for England 2017
- `sex`: 0 female; 1 male
- `age`: in years
- `origin`: 1 White; 2 Black; 3 Asian; 4 Mixed; 5 Other
- `nssec5`: National Statistics Socio-economic classification:
    - 1: Higher managerial, administrative and professional occupations
    - 2: Intermediate occupations
    - 3: Small employers and own account workers
    - 4: Lower supervisory and technical occupations
    - 5: Semi-routine and routine occupations
    - 0: Never worked and long-term unemployed
- `soc2010`: Previous version of the [Standard Occupational Classification]( https://www.ons.gov.uk/methodology/classificationsandstandards/standardoccupationalclassificationsoc/soc2010)
- `sic1d07`: Standard [Industrial Classification of Economic Activities 2007](https://www.ons.gov.uk/methodology/classificationsandstandards/ukstandardindustrialclassificationofeconomicactivities), 1st layer (number corresponding to the letter in alphabetical order)
- `sic2d07`: Standard [Industrial Classification of Economic Activities 2007](https://www.ons.gov.uk/methodology/classificationsandstandards/ukstandardindustrialclassificationofeconomicactivities), 2nd layer 
- Proportion of 24h spent doing different daily activities:
    `punknown` + `pwork` + `pschool` + `pshop` + `pservices` + `pleisure` + `pescort` + `ptransport` = `pnothome`
    `phome` + `pworkhome` = `phometot`
    `pnothome` + `phometot` = 1
- `cvd`: has a cardio-vascular disease (0 or 1)
- `diabetes`: has diabetes (0 or 1)
- `bloodpressure`: has high blood pressure (0 or 1)
- `BMIvg6`: Body Mass Index:
    - Not applicable
    - Underweight: less than 18.5
    - Normal: 18.5 to less than 25
    - Overweight: 25 to less than 30
    - Obese I: 30 to less than 35
    - Obese II: 35 to less than 40
    - Obese III: 40 or more
- `lng`: longitude of the MSOA11CD centroid
- `lat`: latitude of the MSOA11CD centroid

Some other fields were kept from specific projects but are not from official sources and should be used.


## National data

```
businessRegistry.csv
```

Contains a breakdown of all business units (i.e. a single workplace) in England at LSOA scale (smaller than MSOA), estimated by the project contributors from two nomis datasets: [UK Business Counts - local units by industry and employment size band 2020](https://www.nomisweb.co.uk/datasets/idbrlu) and [Business Register and Employment Survey 2015](https://www.nomisweb.co.uk/datasets/newbrespub). Each item contains the `size`  of the unit and its main `sic1d07` code in reference to standard [Industrial Classification of Economic Activities 2007](https://www.ons.gov.uk/methodology/classificationsandstandards/ukstandardindustrialclassificationofeconomicactivities) (number corresponding to the letter in alphabetical order). It is used to compute commuting flows.

```
MSOAS_shp.tar.gz
```

Is a simple shapefile taken from ONS [boundaries](https://geoportal.statistics.gov.uk/).

```
QUANT_RAMP.tar.gz
```

See: Milton R, Batty M, Dennett A, dedicated [RAMP Spatial Interaction Model GitHub repository](https://github.com/maptube/QUANT_RAMP). It is used to compute the flows towards schools and retail.

```
timeAtHomeIncreaseCTY.csv
```

This file is a subset from [Google COVID-19 Community Mobility Reports](https://www.google.com/covid19/mobility/), cropped to England. It describes the daily reduction in mobility, average at county level, due to lockdown and other COVID-19 restrictions between the 15<sup>th</sup> of February 2020 and 2<sup>nd</sup> of March 2021. Missing values have been replaced by the national average. These values can be used directly to reduce `pnothome` and increase `phometot` (and their sub-categories) to simulate more accurately the period.
