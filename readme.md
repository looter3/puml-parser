
# PlantUML Parser
## _Parse UML Diagrams into Source Code_

The "PlantUML Parser" (also known as puml-parser) is a desktop application that allows you to convert class diagrams written with [PlantUML] into native source code (currently, only Java is supported). The backend of the application is written in Rust, while the frontend is built with React, thanks to [Tauri].

The project architecture consists of three main modules:

- **Backend Module**: Packaged as a Rust library (name: _puml_), it handles the business logic and parsing operations (located in the root folder _../parse-puml/src/_).
- **Tauri Module**: A Rust binary crate that serves as the engine of the application. It is also responsible for defining commands that wrap backend functions to be exposed via the _invoke_ method.
- **Frontend Module**: A pure frontend module built with TypeScript, React, and Tailwind CSS.

## Features

- Import a PlantUML file and obtain the corresponding class source code.
- **TODO**: In the `FilePathInput` page, allow the user to choose the destination language (e.g., C#, Java, etc.) (Frontend - FilePathInput component).
- **TODO**: Export classes to a file (Frontend) - We chose to implement this on the frontend so users can edit classes after parsing.
- **TODO**: Add package and import statements at the top of the files (Backend - Source Code Generator Module).
- **TODO**: Add support for additional languages (Backend).
- **TODO**: Integrate with existing projects by adding generated classes or create new projects from diagrams.
- **TODO**: Improve the graphical interface.

## Build

To build the application, run the following command:

```sh
cargo tauri build
```

from the _../parser-puml_ directory.

To build only the frontend, run:

```sh
cd frontend
npm run build
```

## Debug

To debug the application, you need to start the frontend first:

```sh
cd frontend
npm start
```

Then, start the backend:

```sh
cd ..
cargo run
```

You can now place breakpoints in the Rust code using your IDE, and open the console from the desktop application, which also allows debugging in the TypeScript files.

[PlantUML]: <https://plantuml.com/>
[Tauri]: <https://v2.tauri.app/>
