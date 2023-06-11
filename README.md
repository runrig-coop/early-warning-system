# Early Warning System
An auditing dashboard that reports when and where (in terms of relative urgency) an on-site farm visit or other action may be required. Through a combination of automated, manual and verbal survey submissions, the status of each farm can be updated with regular crop assessments and reports on general field health.

## Development Quick Start
First, make sure you have [Rust], [Node.js], and the full suite of [Tauri prerequisites] installed on your local development machine.

Then after cloning this repository, install the Node/JavaScript dependencies with [npm] (or the package manager of your choice), and start the Tauri development environment:

```sh
npm i               # Install JS dependencies
npm run tauri dev   # Start the Tauri development environment

# Alternatively, you can run the app in a browser-based dev env:
npm run dev
```

## Data Model
The data model consists of three tables representing physical items (farm, land, and planting) and three tables representing events or logs recorded by members (measurement, quantitative_measurement, and qualitative_measurement).

![Data model of the RGO app](images/data_model.png)

[Rust]: https://www.rust-lang.org/
[Node.js]: https://nodejs.org/
[Tauri prerequisites]: https://tauri.app/v1/guides/getting-started/prerequisites
[npm]: https://www.npmjs.com/
