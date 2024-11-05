## Paidy: Simple Restaurant API
_https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md_

Rest Compliant API Hosts two Resources for clients:
- Table Resource (restaurant table)
- Item Resource  (ordered item for a table)

For a restaurant, following actions can be performed using the implemented REST APIs:
- Add Item(s) to a table
- List Item(s) of a table
- Remove Item from a table

### Requirements Covered:
- ✅ The server API MUST fully follow REST API principles and present a set of HTTP endpoints to connect to.
  
  **_REST Compliant API responses are present in the last section of this file_**

  1. Add Item(s) to Table `(POST Request, 201 Created)`
  ```bash
    curl --location 'localhost:8080/tables/1/items' \
    --header 'Content-Type: application/json' \
    --data '{
        "items_names": [
            "拉麺",
            "うどん",
            "そば",
            "そうめん",
            "中華そば",
            "とんかつ",
            "からあげ",
            "やきにく",
            "カレーライス",
            "オムライス"
        ]
    }'
  ```
  2. Get Table Resource `(GET Request, 200 OK)`
  ```bash
    curl --location 'localhost:8080/tables/1'
  ```
  3. Get Table Item resource `(GET Request, 200 OK)`
  ```bash
    curl --location 'localhost:8080/tables/1/items/2'
  ```
  4. Get All Items of a Table `(GET Request, 200 OK)`
  ```bash
    curl --location 'localhost:8080/tables/1/items'
  ```
  5. Delete Item from a Table `(DELETE Request, 200 OK)`
  ```bash
    curl --location --request DELETE 'localhost:8080/tables/1/items/4' 
  ```
  6. Health Check `(GET Request, 200 OK)`
  ```bash
    curl --location 'localhost:8080/health_check'
  ```

- ✅ The client (the restaurant staff “devices” making the requests) MUST be able to: 
  - add one or more items with a table number,
    ```bash
    curl --location 'localhost:8080/tables/1/items' \
        --header 'Content-Type: application/json' \
        --data '{
            "items_names": [
                "拉麺",
                "うどん",
                "そば",
                "そうめん",
                "中華そば",
                "とんかつ",
                "からあげ",
                "やきにく",
                "カレーライス",
                "オムライス"
            ]
        }'
    ```
  - remove an item for a table,
    ```bash
    curl --location --request DELETE 'localhost:8080/tables/1/items/4'
    ```
  - and query the items still remaining for a table.
    ```bash
    curl --location 'localhost:8080/tables/1/items'
    ```    


- ✅ The application MUST, upon creation request, 
  - store the item,
  - the table number, 
  - and how long the item will take to cook.


- ✅ The application MUST, upon deletion request, 
  - remove a specified item for a specified table number.
  ```bash
  curl --location --request DELETE 'localhost:8080/tables/1/items/4'
  ```
  

- ✅ The application MUST, upon query request, 
  - show all items for a specified table number.
  ```bash
  curl --location 'localhost:8080/tables/1/items'
  ```

  
- ✅ The application MUST, upon query request, 
  - show a specified item for a specified table number.
  ```bash
  curl --location 'localhost:8080/tables/1/items/2'
  ```

  
- ✅ The application MUST accept at least 10 simultaneous incoming add/remove/query requests.
 ![Alt text](docs/handling_10_concurrent_requests.png)

- ✅ The client MAY limit the number of specific tables in its requests to a finite set (at least 100).


- ✅ The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.

 
- ✅ The application MAY keep the length of time for the item to prepare static

---
### Tools
| Name   | Version |
|--------|---------|
| docker | 27.2.0  |
| hurl   | 5.0.1   |
| k6     | v0.54.0 |
| mysql  | 8.4.2   |
| rustc  | 1.81.0  |

---
#### How to run
- Inside the folder, `simple-restaurant-api`, execute: 
 ```bash
docker-compose up
```
Although, the application boots up in no time, this project keeps a small wait *between* mysql-database-initialization 
and rust-application-startup.

---

#### How to Test APIs
- install hurl by running:
```bash
brew install hurl
```
- inside testing folder execute
```bash
hurl --test .
```
this will test all apis, and their cases, one by one

---

#### How to Stress Test APIs
- install k6 by running:
```bash
brew install k6
```
- Stress test by executing k6 command with appropriate parameters which are defined inside the .js file for stress test
```bash
k6 run add_items.js
```
- k6's configurable parameters:
```bash
vus: number of concurrent requests
duration: amount of time to continue hammering the API endpoint
```

---
#### API Responses 
To Show REST Compliance and HATEAOS presence.

1. Add Items to Table `POST localhost:8080/tables/1/items`
    ```json
    {
      "status": "success",
      "message": "Added 10 item(s) to table number 1",
      "items_ids": [
          1,
          2,
          3,
          4,
          5,
          6,
          7,
          8,
          9,
          10
      ]
    }
    ```

2. Get Table Resource `GET localhost:8080/tables/1`
    ```json
    {
       "table_number": 1,
       "items": [
           {
               "item_id": 1,
               "table_number": 1,
               "item_name": "拉麺",
               "ordered_on": "2024-11-05 19:40:58",
               "prepare_minutes": 9,
               "_links": [
                   {
                       "href": "/tables/1/items/1",
                       "rel": "self",
                       "method": "Get"
                   },
                   {
                       "href": "/tables/1/items/1",
                       "rel": "delete",
                       "method": "Delete"
                   },
                   {
                       "href": "/tables/1",
                       "rel": "table",
                       "method": "Get"
                   }
               ]
           },
           {
               "item_id": 2,
               "table_number": 1,
               "item_name": "うどん",
               "ordered_on": "2024-11-05 19:40:58",
               "prepare_minutes": 11,
               "_links": [
                   {
                       "href": "/tables/1/items/2",
                       "rel": "self",
                       "method": "Get"
                   },
                   {
                       "href": "/tables/1/items/2",
                       "rel": "delete",
                       "method": "Delete"
                   },
                   {
                       "href": "/tables/1",
                       "rel": "table",
                       "method": "Get"
                   }
               ]
           }
       ],
       "_links": [
           {
               "href": "/tables/1",
               "rel": "self",
               "method": "Get"
           },
           {
               "href": "/tables/1",
               "rel": "add_items",
               "method": "Put"
           }
       ]
     }
     ```

3. GET Item Resource `GET localhost:8080/tables/1/items/2`
    ```json
    {
      "item_id": 2,
      "table_number": 1,
      "item_name": "うどん",
      "ordered_on": "2024-11-05 19:40:58",
      "prepare_minutes": 11,
      "_links": [
          {
              "href": "/tables/1/items/2",
              "rel": "self",
              "method": "Get"
          },
          {
              "href": "/tables/1/items/2",
              "rel": "delete",
              "method": "Delete"
          },
          {
              "href": "/tables/1",
              "rel": "table",
              "method": "Get"
          }
      ]
    }
    ```
4. GET All Items of a Table `GET localhost:8080/tables/1/items`
    ```json
    [
        {
            "item_id": 1,
            "table_number": 1,
            "item_name": "拉麺",
            "ordered_on": "2024-11-05 19:40:58",
            "prepare_minutes": 9,
            "_links": [
                {
                    "href": "/tables/1/items/1",
                    "rel": "self",
                    "method": "Get"
                },
                {
                    "href": "/tables/1/items/1",
                    "rel": "delete",
                    "method": "Delete"
                },
                {
                    "href": "/tables/1",
                    "rel": "table",
                    "method": "Get"
                }
            ]
        },
        {
            "item_id": 2,
            "table_number": 1,
            "item_name": "うどん",
            "ordered_on": "2024-11-05 19:40:58",
            "prepare_minutes": 11,
            "_links": [
                {
                    "href": "/tables/1/items/2",
                    "rel": "self",
                    "method": "Get"
                },
                {
                    "href": "/tables/1/items/2",
                    "rel": "delete",
                    "method": "Delete"
                },
                {
                    "href": "/tables/1",
                    "rel": "table",
                    "method": "Get"
                }
            ]
        }
    ]
    ```
5. Remove Item Resource `GET localhost:8080/tables/1/items/4`
    ```json
    {
        "status": "success",
        "message": "Removed table item with item_id 4"
    }
    ```
6. Basic Health Check `GET localhost:8080/health_check`
    ```text
    Health OK at 2024-11-05][19:42:31
    ```