# Rust Actix Web REST API to Import Nested Array of Items in PostgreSQL database 

## Overview

This project is a REST API built using Rust with Actix Web framework and Diesel ORM. The API is specifically designed to import a 2D array (nested array) of point of sale (POS) items from the front-end into a PostgreSQL database.

## Prerequisites

- Rust and Cargo
- PostgreSQL
- Diesel CLI

## Setup

1. **Create a Database**

    Ensure you have a PostgreSQL database set up.

2. **Connect the Database Using .env File**

    Create a `.env` file in the root directory with your database connection details:

    ```env
    DATABASE_URL=postgresql://superuser:superpassword@localhost:5432/pos_items_database
    ```

3. **Run Migration**

    ```sh
    diesel migration run
    ```

4. **Run the Application**

    ```sh
    cargo run
    ```

    The server will start at `http://127.0.0.1:8080`.

## Endpoints

- **Health Check**

    ```http
    GET /health
    ```

    Response:

    ```json
    {
        "message": "Everything is working fine"
    }
    ```

- **Import POS Items**

    ```http
    POST /api/pos-items
    ```

    Request Body:

    ```json
    {
        "outer_array": [
            {
                "code": "A001",
                "description": "Frame A001",
                "category": "Material",
                "sub_category": "Metal"
            }
            
        ]
    }
    ```

    Response:

    ```json
    "POS items import successful."
    ```

