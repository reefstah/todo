
Dependency layout

```mermaid
graph TD;
    usecases-->entities;
    fs_repository-->entities;
    cli-->entities;
    cli-->fs_repository;
    cli-->usecases;
```
