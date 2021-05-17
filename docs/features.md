Features
========
The features I am going to implemented.

* Follow DDD patterns
  * [ ] Repository for database operations
  * [ ] Model for business logic
* Testing
  * [x] Integration test sample
  * [ ] Rollback transactinon for functional test
* [x] Using Postgresql instead of MySQL
  
  Sorry I don't use MySQL, uuid is internally handled by Postgresql, I need a way to create the id for an entity before persisting it into database.


* JWT Authentication
* Integrate GraphQL
* Multiple languages
  
  * [x] [Fluent](https://blog.logrocket.com/rust-internationalization-localization-and-translation/#gettext-rs)
  
      It seems Fluent is more promising for website, check the link for details.

  * [x]  Negotiate language ontop of Fluent
  * [ ]  Allow to translate for validator

* Dependency injection
  
  [Shaku](https://docs.rs/shaku/0.6.1/shaku/guide/index.html) Shaku is a compile time dependency injection Rust library