# Materials Endpoints Expected Data

This document outlines the expected request and response data for the API endpoints related to the `materials` application.

---

### Create Material

*   **URL:** `/api/course/<uuid:course_id>/materials/create/`
*   **Method:** `POST`

    **Request (POST):**
    ```json
    {
        "title": "Lecture Notes",
        "file": "<binary file upload>",
        "type": "document",
        "labels": [
            {
                "label": "uuid-of-label",
                "number": 1
            }
        ]
    }
    ```
    OR (for URL type)
    ```json
    {
        "title": "Tutorial Video",
        "url": "https://youtu.be/xxxxxx",
        "type": "url",
        "labels": []
    }
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "id": "uuid-of-material",
        "title": "Lecture Notes",
        "file": "/media/materials/<course_id>/file.pdf",
        "url": null,
        "type": "document",
        "created_at": "timestamp",
        "updated_at": "timestamp",
        "owner": "uuid-of-user"
    }
    ```

### List Materials

*   **URL:** `/api/course/<uuid:course_id>/materials/`
*   **Method:** `GET`

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "id": "uuid-of-material",
            "title": "Lecture Notes",
            "file": "/media/materials/<course_id>/file.pdf",
            "url": null,
            "type": "document",
            "created_at": "timestamp",
            "updated_at": "timestamp",
            "owner": "uuid-of-user"
        },
        {
            "id": "uuid-of-material-2",
            "title": "Tutorial Video",
            "file": null,
            "url": "https://youtu.be/xxxxxx",
            "type": "url",
            "created_at": "timestamp",
            "updated_at": "timestamp",
            "owner": "uuid-of-user"
        }
    ]
    ```

### Update / Delete Material

*   **URL:** `/api/materials/<uuid:material_id>/`
*   **Methods:** `PUT`, `DELETE`

    **PUT Request:**
    ```json
    {
        "title": "Updated Lecture Notes"
    }
    ```

    **PUT Response (200 OK):**
    ```json
    {
        "id": "uuid-of-material",
        "title": "Updated Lecture Notes",
        "file": "/media/materials/<course_id>/file.pdf",
        "url": null,
        "type": "document",
        "created_at": "timestamp",
        "updated_at": "timestamp",
        "owner": "uuid-of-user"
    }
    ```

    **Response (DELETE - 204 No Content):** (No content returned on successful deletion)

### Material Labels

#### Add / Retrieve Labels for a Material

*   **URL:** `/api/materials/<uuid:material_id>/labels/`
*   **Methods:** `GET`, `PUT` (Note: Documentation says POST, but code implements PUT for updating/replacing)

    **PUT Request:**
    ```json
    {
        "labels": [
            {
                "label": "uuid-of-label",
                "number": 1
            },
            {
                "label": "uuid-of-another-label",
                "number": 2
            }
        ]
    }
    ```

    **GET Response (200 OK):**
    ```json
    [
        {
            "label": {
                "id": "uuid-of-label",
                "name": "Priority",
                "group": "uuid-of-group",
                "min_value": 1,
                "max_value": 5
            },
            "number": 1
        },
        {
            "label": {
                "id": "uuid-of-another-label",
                "name": "Difficulty",
                "group": "uuid-of-group",
                "min_value": 1,
                "max_value": 3
            },
            "number": 2
        }
    ]
    ```

    **PUT Response (201 CREATED):** (If new labels are added/replaced)
    ```json
    [
        {
            "label": "uuid-of-label",
            "number": 1,
            "material": "uuid-of-material"
        },
        {
            "label": "uuid-of-another-label",
            "number": 2,
            "material": "uuid-of-material"
        }
    ]
    ```
    **PUT Response (204 No Content):** (If `labels` array is empty, indicating removal of all labels)

#### List Materials by Label

*   **URL:** `/api/course/<uuid:course_id>/materials/labels/<label_id>/`
*   **Method:** `GET`

    **Response (GET - 200 OK):**
    ```json
    {
        "label_name": "Priority",
        "materials": {
            "1": [
                {
                    "material": {
                        "id": "uuid-of-material",
                        "title": "Lecture Notes",
                        "file": "/media/materials/<course_id>/file.pdf",
                        "url": null,
                        "type": "document",
                        "created_at": "timestamp",
                        "updated_at": "timestamp",
                        "owner": "uuid-of-user"
                    }
                }
            ],
            "2": [
                {
                    "material": {
                        "id": "uuid-of-material-2",
                        "title": "Tutorial Video",
                        "file": null,
                        "url": "https://youtu.be/xxxxxx",
                        "type": "url",
                        "created_at": "timestamp",
                        "updated_at": "timestamp",
                        "owner": "uuid-of-user"
                    }
                }
            ]
        }
    }
    ```

### Material Comments

#### Create Comment

*   **URL:** `/api/comments/create/`
*   **Method:** `POST`

    **Request (POST):**
    ```json
    {
        "material": "uuid-of-material",
        "Content": "This lecture was very helpful!"
    }
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "id": "uuid-of-comment",
        "material": "uuid-of-material",
        "User": "uuid-of-user",
        "Content": "This lecture was very helpful!",
        "CreatedAt": "timestamp"
    }
    ```

#### List Comments for a Material

*   **URL:** `/api/materials/<uuid:material_id>/comments/`
*   **Method:** `GET`

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "id": "uuid-of-comment",
            "material": "uuid-of-material",
            "User": "uuid-of-user",
            "Content": "This lecture was very helpful!",
            "CreatedAt": "timestamp"
        },
        {
            "id": "uuid-of-comment-2",
            "material": "uuid-of-material",
            "User": "uuid-of-user-2",
            "Content": "I have a question regarding slide 5.",
            "CreatedAt": "timestamp"
        }
    ]
    ```

#### Update / Delete Comment

*   **URL:** `/api/comments/<uuid:comment_id>/`
*   **Methods:** `PUT`, `DELETE`

    **PUT Request:**
    ```json
    {
        "Content": "Updated comment text."
    }
    ```

    **PUT Response (200 OK):**
    ```json
    {
        "id": "uuid-of-comment",
        "material": "uuid-of-material",
        "User": "uuid-of-user",
        "Content": "Updated comment text.",
        "CreatedAt": "timestamp"
    }
    ```

    **Response (DELETE - 204 No Content):** (No content returned on successful deletion)