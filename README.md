# Early Warning System - Richland Gro-Op
An early warning system for Richland Gro-Op to coordinate field health.

This gives a dashboard and quick visual reference for coordinating farms and fields within the Richland Gro-Op group.

To run, use 
```
npm run tauri dev
```

## Data Model
The data model consists of three tables representing physical items (farm, land, and planting) and three tables representing events or logs recorded by members (measurement, quantitative_measurement, and qualitative_measurement).

![Data model of the RGO app](images/data_model.png)
