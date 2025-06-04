# Groups & Courses Endpoints Expected Data

This document outlines the expected request and response data for the API endpoints related to the `groups_courses` application.

---

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