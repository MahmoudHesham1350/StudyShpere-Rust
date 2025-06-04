# Users Endpoints Expected Data

This document outlines the expected request and response data for the API endpoints related to the `users` application.

---

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