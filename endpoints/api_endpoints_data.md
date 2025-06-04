# API Endpoints Expected Data

This document outlines the expected request and response data for the various API endpoints identified in the codebase.

---

## Groups & Courses Endpoints

### Group Endpoints

#### List and Create Groups

*   **URL:** `/api/groups/`
*   **Method:** `POST` (Create Group)

    **Request (POST):**
    ```json
    {
        "name": "Study Group",
        "description": "A group for studying",
        "join_type": "open",
        "post_permission": "members",
        "edit_permissions": "admins"
    }
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "id": "uuid",
        "owner": "user_id",
        "name": "Study Group",
        "description": "A group for studying",
        "join_type": "open",
        "post_permission": "members",
        "edit_permissions": "admins",
        "created_at": "timestamp"
    }
    ```

*   **URL:** `/api/groups/list/`
*   **Method:** `GET` (List All Groups)

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "id": "uuid",
            "owner": "user_id",
            "name": "Study Group",
            "description": "A group for studying",
            "join_type": "open",
            "post_permission": "members",
            "edit_permissions": "admins",
            "created_at": "timestamp"
        }
    ]
    ```

#### Retrieve, Update, and Delete Group

*   **URL:** `/api/groups/<uuid:group_id>/`
*   **Method:** `GET`, `PUT`, `DELETE`

    **Response (GET - 200 OK):**
    ```json
    {
        "id": "uuid",
        "owner": "user_id",
        "name": "Study Group",
        "description": "A group for studying",
        "join_type": "open",
        "post_permission": "members",
        "edit_permissions": "admins",
        "created_at": "timestamp",
        "members": [
            {
                "user": {
                    "id": "user_id",
                    "username": "username"
                },
                "user_role": "member",
                "joined_at": "timestamp"
            }
        ],
        "courses": [
            {
                "id": "uuid",
                "group": {
                    "id": "uuid",
                    "name": "Study Group"
                },
                "name": "Course Name",
                "description": "Course Description",
                "created_at": "timestamp"
            }
        ]
    }
    ```

    **Request (PUT):** (Example, full request body would be similar to POST create group)
    ```json
    {
        "name": "Updated Study Group Name",
        "description": "Updated description for the study group"
    }
    ```

    **Response (PUT - 200 OK):** (Similar to GET response for group detail)
    ```json
    {
        "id": "uuid",
        "owner": "user_id",
        "name": "Updated Study Group Name",
        "description": "Updated description for the study group",
        "join_type": "open",
        "post_permission": "members",
        "edit_permissions": "admins",
        "created_at": "timestamp",
        "members": [],
        "courses": []
    }
    ```

    **Response (DELETE - 204 No Content):** (No content returned on successful deletion)

### Group Member Endpoints

#### List and Create Group Members

*   **URL:** `/api/groups/<uuid:group_id>/members/`
*   **Method:** `GET` (List Group Members)

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "user": {
                "id": "user_id",
                "username": "username"
            },
            "user_role": "member",
            "joined_at": "timestamp"
        }
    ]
    ```

*   **URL:** `/api/groups/<uuid:group_id>/members/create/`
*   **Method:** `POST` (Create Group Member)

    **Request (POST):**
    ```json
    {
        "user": "user_id"
    }
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "user": {
            "id": "user_id",
            "username": "username"
        },
        "user_role": "member",
        "joined_at": "timestamp"
    }
    ```

#### Retrieve, Update, and Delete Group Member

*   **URL:** `/api/groups/<uuid:group_id>/members/<uuid:user_id>/`
*   **Method:** `GET`, `PUT`, `DELETE`

    **Response (GET - 200 OK):**
    ```json
    {
        "user": {
            "id": "user_id",
            "username": "username"
        },
        "user_role": "member",
        "joined_at": "timestamp"
    }
    ```

    **Request (PUT):** (Example, typically for updating `user_role`)
    ```json
    {
        "user_role": "moderator"
    }
    ```

    **Response (PUT - 200 OK):** (Similar to GET response for group member detail)
    ```json
    {
        "user": {
            "id": "user_id",
            "username": "username"
        },
        "user_role": "moderator",
        "joined_at": "timestamp"
    }
    ```

    **Response (DELETE - 204 No Content):** (No content returned on successful deletion)

### Join Request Endpoints

#### List and Create Join Requests

*   **URL:** `/api/groups/<uuid:group_id>/join-requests/`
*   **Method:** `GET`, `POST`

    **Request (POST):**
    ```json
    {
        "group": "group_id",
        "user": "user_id"
    }
    ```

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "user": {
                "id": "user_id",
                "username": "username"
            },
            "created_at": "timestamp"
        }
    ]
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "user": {
            "id": "user_id",
            "username": "username"
        },
        "created_at": "timestamp"
    }
    ```

#### Respond to Join Request

*   **URL:** `/api/join-requests/<uuid:join_request_id>/`
*   **Method:** `POST`

    **Request (POST):**
    ```json
    {
        "action": "accept"
    }
    ```
    OR
    ```json
    {
        "action": "decline"
    }
    ```

    **Response (POST - 200 OK):**
    ```json
    {
        "message": "User added to group"
    }
    ```
    OR
    ```json
    {
        "message": "Join request declined"
    }
    ```

### Course Endpoints

#### List and Create Courses

*   **URL:** `/api/groups/<uuid:group_id>/courses/`
*   **Method:** `GET`, `POST`

    **Request (POST):**
    ```json
    {
        "name": "Course Name",
        "description": "Course Description"
    }
    ```

    **Response (GET - 200 OK):**
    ```json
    [
        {
            "id": "uuid",
            "group": {
                "id": "uuid",
                "name": "Study Group"
            },
            "name": "Course Name",
            "description": "Course Description",
            "created_at": "timestamp"
        }
    ]
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "id": "uuid",
        "group": {
            "id": "uuid",
            "name": "Study Group"
        },
        "name": "Course Name",
        "description": "Course Description",
        "created_at": "timestamp"
    }
    ```

#### Retrieve, Update, and Delete Course

*   **URL:** `/api/courses/<uuid:course_id>/`
*   **Method:** `GET`, `PUT`, `DELETE`

    **Response (GET - 200 OK):**
    ```json
    {
        "id": "uuid",
        "group": {
            "id": "uuid",
            "name": "Study Group"
        },
        "name": "Course Name",
        "description": "Course Description",
        "created_at": "timestamp"
    }
    ```

    **Request (PUT):**
    ```json
    {
        "name": "Updated Course Name",
        "description": "Updated Course Description"
    }
    ```

    **Response (PUT - 200 OK):** (Similar to GET response for course detail)
    ```json
    {
        "id": "uuid",
        "group": {
            "id": "uuid",
            "name": "Study Group"
        },
        "name": "Updated Course Name",
        "description": "Updated Course Description",
        "created_at": "timestamp"
    }
    ```

    **Response (DELETE - 204 No Content):** (No content returned on successful deletion)

---

## Materials Endpoints

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

---

## Users Endpoints

### User Registration

*   **URL:** `/api/user/register/`
*   **Method:** `POST`

    **Request (POST):**
    ```json
    {
        "email": "user@example.com",
        "username": "newuser",
        "password": "StrongPassword123!",
        "confirm_password": "StrongPassword123!"
    }
    ```

    **Response (POST - 201 CREATED):**
    ```json
    {
        "user": "uuid-of-user"
    }
    ```
    (Also sets `access_token` and `refresh_token` as HttpOnly cookies)

### User Login

*   **URL:** `/api/user/login/`
*   **Method:** `POST`

    **Request (POST):**
    ```json
    {
        "email": "user@example.com",
        "password": "StrongPassword123!"
    }
    ```

    **Response (POST - 200 OK):**
    ```json
    {
        "user": "uuid-of-user"
    }
    ```
    (Also sets `access_token` and `refresh_token` as HttpOnly cookies)

### User Logout

*   **URL:** `/api/user/logout/`
*   **Method:** `POST`

    **Request (POST):** (No request body needed, token is read from cookie)

    **Response (POST - 205 RESET CONTENT):**
    ```json
    {
        "message": "Logged out successfully"
    }
    ```
    (Clears `access_token` and `refresh_token` cookies)

### Refresh Access Token

*   **URL:** `/api/user/token/refresh/`
*   **Method:** `POST`

    **Request (POST):** (No request body needed, refresh token is read from cookie)

    **Response (POST - 200 OK):**
    (Sets a new `access_token` as HttpOnly cookie)

### Verify Token

*   **URL:** `/api/user/token/verify/`
*   **Method:** `POST`

    **Request (POST):** (No request body needed, refresh token is read from cookie)

    **Response (POST - 200 OK):**
    ```json
    {
        "message": "Token is valid"
    }
    ```

### User Profile

*   **URL:** `/api/user/profile/`
*   **Methods:** `GET`, `PUT`

    **Response (GET - 200 OK):**
    ```json
    {
        "user": {
            "id": "uuid-of-user",
            "email": "user@example.com",
            "username": "newuser"
        },
        "bio": "User's biography",
        "profile_picture": "url-to-profile-picture",
        "Affiliation": "User's affiliation"
    }
    ```

    **PUT Request:**
    ```json
    {
        "bio": "Updated biography text.",
        "Affiliation": "New Affiliation"
    }
    ```

    **PUT Response (200 OK):** (Similar to GET response for profile)
    ```json
    {
        "user": {
            "id": "uuid-of-user",
            "email": "user@example.com",
            "username": "newuser"
        },
        "bio": "Updated biography text.",
        "profile_picture": "url-to-profile-picture",
        "Affiliation": "New Affiliation"
    }