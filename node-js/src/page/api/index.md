# Building & Consuming APIs

**APIs (Application Programming Interfaces)** are essential tools in software development, acting as intermediaries that enable different software applications to communicate with each other. APIs provide a set of rules and protocols that allow software programs to interact, send, and receive data.

### Key Concepts:

1. **Request and Response**: When an API is called, it sends a request to a server, which processes it and sends back a response. This process typically follows HTTP (Hypertext Transfer Protocol) standards.
2. **Endpoints**: An API endpoint is a specific path where resources are accessed or manipulated. Each endpoint typically represents a distinct operation in the API.
3. **Methods**: API requests usually involve one of the following HTTP methods:
   - **GET**: Retrieve data from the server.
   - **POST**: Send data to the server (often to create new resources).
   - **PUT**: Update existing data on the server.
   - **DELETE**: Remove data from the server.
4. **Headers**: Additional information sent with API requests or responses. They can include content types, authorization tokens, etc.

5. **Authentication**: Many APIs require authentication to ensure that only authorized users or systems can access the resources. This could involve tokens, API keys, OAuth, etc.

6. **Rate Limiting**: APIs often enforce limits on the number of requests that can be made within a specific period to prevent abuse or overloading of the system.

7. **Status Codes**: API responses are often accompanied by status codes, indicating whether the request was successful or if there was an error (e.g., `200 OK`, `404 Not Found`, `500 Internal Server Error`).

### Example:

An API endpoint for retrieving user data might look like this:

```http
GET https://api.example.com/users/123
```

- **Request**: Sends a GET request to the server to retrieve the user with ID `123`.
- **Response**: The server returns user data in JSON format.

```json
{
  "id": 123,
  "name": "John Doe",
  "email": "john@example.com"
}
```

### Questions & Answers on APIs:

---

### 1. **What is an API, and how does it work?**

- **Answer**: An API (Application Programming Interface) is a set of rules and protocols that allows software programs to communicate with each other. It works by sending an HTTP request to an API endpoint, which processes the request on the server and returns the appropriate response, typically in JSON or XML format.

---

### 2. **What is REST, and how does it relate to APIs?**

- **Answer**: REST (Representational State Transfer) is an architectural style for building APIs. It uses standard HTTP methods (GET, POST, PUT, DELETE) and follows principles such as statelessness, cacheability, and a client-server relationship. RESTful APIs follow these guidelines to ensure scalability and performance.

---

### 3. **What are the common HTTP methods used in APIs?**

- **Answer**: The most common HTTP methods in APIs are:
  - **GET**: Retrieve data from the server.
  - **POST**: Submit data to be processed, often to create a resource.
  - **PUT**: Update an existing resource.
  - **DELETE**: Remove a resource.
  - **PATCH**: Partially update a resource.

---

### 4. **What is the purpose of API authentication, and how is it commonly implemented?**

- **Answer**: API authentication ensures that only authorized users or applications can access the API. It is commonly implemented using API keys, bearer tokens, or OAuth. For example, an API key might be passed in the headers or URL to authenticate the request.

---

### 5. **What is a RESTful API, and what are its key principles?**

- **Answer**: A RESTful API follows the principles of REST, which include:
  - **Statelessness**: The server does not store the state of the client.
  - **Client-server**: Separation between the client and the server.
  - **Uniform Interface**: Standardized API responses and structures.
  - **Cacheability**: Resources should be cacheable to improve performance.

---

### 6. **What is the difference between a REST API and a SOAP API?**

- **Answer**:
  - **REST API**: REST is a lightweight, flexible architecture using HTTP protocols and typically exchanging data in JSON format. It is stateless and has faster performance.
  - **SOAP API**: SOAP (Simple Object Access Protocol) is a protocol that uses XML for message format and relies on more rigid standards. It provides more robust security features and is stateful.

---

### 7. **How do you handle rate limiting in an API?**

- **Answer**: Rate limiting restricts the number of API requests a user or application can make within a certain period to avoid overloading the server. APIs typically return HTTP status codes like `429 Too Many Requests` when the limit is exceeded. Handling this could involve retry mechanisms, throttling, or queuing requests.

---

### 8. **What is an API endpoint?**

- **Answer**: An API endpoint is a specific URL where an API resource can be accessed. Each endpoint is typically tied to a particular function of the API (e.g., retrieving data, updating a resource). For example:
  ```
  GET https://api.example.com/users/123
  ```
  This is an endpoint to retrieve the user with ID `123`.

---

### 9. **What is a query parameter in an API, and how is it used?**

- **Answer**: A query parameter is part of the URL that provides additional information to the server. It typically follows a `?` in the URL. For example:
  ```
  GET https://api.example.com/users?age=25&city=London
  ```
  In this example, `age=25` and `city=London` are query parameters used to filter users.

---

### 10. **What are API status codes? Give examples.**

- **Answer**: API status codes are returned by the server to indicate the outcome of an API request. Some common examples:
  - **200 OK**: The request was successful.
  - **201 Created**: The resource was successfully created.
  - **400 Bad Request**: The request was invalid or cannot be processed.
  - **401 Unauthorized**: Authentication is required.
  - **404 Not Found**: The requested resource could not be found.
  - **500 Internal Server Error**: An error occurred on the server.

---

### 11. **What is CORS, and why is it important for APIs?**

- **Answer**: CORS (Cross-Origin Resource Sharing) is a security feature that restricts web applications from making requests to a different domain than the one that served the web page. It's important for APIs to implement proper CORS policies to prevent unauthorized access and control which domains can interact with the API.

---

### 12. **What is an API Gateway?**

- **Answer**: An API Gateway is a server that acts as an intermediary between clients and microservices. It manages all API requests and enforces policies such as authentication, rate limiting, and load balancing. Popular API gateways include AWS API Gateway and Kong.

---

### 13. **How do you paginate API responses?**

- **Answer**: Pagination in APIs is implemented to manage large datasets. It breaks the response into smaller chunks and returns them incrementally. A common approach uses query parameters like `?page=2&limit=50` to fetch the second page of 50 results. The response usually includes metadata about total pages and records.

---

### 14. **What is GraphQL, and how does it differ from REST?**

- **Answer**: **GraphQL** is a query language for APIs that allows clients to request specific data. Unlike REST, which requires multiple endpoints, GraphQL uses a single endpoint and enables the client to specify exactly what data they need. This makes it more efficient, especially for complex queries or large datasets.

---

### 15. **What is HATEOAS in REST APIs?**

- **Answer**: HATEOAS (Hypermedia As The Engine Of Application State) is a constraint of REST architecture. It means that a client interacts with the application entirely through hypermedia provided dynamically by the server. Essentially, the API response includes links to other API endpoints relevant to the resource, allowing dynamic interaction with the API without prior knowledge of the entire system.

**Example**:

```json
{
  "id": 1,
  "name": "John Doe",
  "links": [
    { "rel": "self", "href": "/users/1" },
    { "rel": "orders", "href": "/users/1/orders" }
  ]
}
```

In this example, the API response provides additional links that the client can follow to related resources, such as the user's orders.

---

### 16. **What are webhooks, and how do they relate to APIs?**

- **Answer**: Webhooks are user-defined HTTP callbacks that are triggered by specific events in a system. They enable one system to send real-time data to another when a specific event occurs. Unlike traditional APIs where you poll for changes, webhooks automatically push data when an event happens, reducing overhead and improving efficiency.

**Example**: In a payment system, a webhook might notify your application when a transaction is completed:

```
POST https://example.com/webhooks/payment-received
{
  "transaction_id": "123456",
  "amount": 100.00,
  "status": "completed"
}
```

---

### 17. **What is OAuth, and how is it used in APIs?**

- **Answer**: **OAuth** (Open Authorization) is an open standard for access delegation, commonly used to grant websites or applications limited access to a user’s data without exposing their credentials. OAuth provides an authorization token that third-party services use to interact with an API on behalf of the user.

**Example Flow**:

1.  User logs in with Google (OAuth provider).
2.  Google provides an access token to the third-party app.
3.  The app uses the access token to access Google services on behalf of the user.

---

### 18. **How do you handle errors in API responses?**

- **Answer**: Proper error handling in APIs involves:
  - **Returning appropriate HTTP status codes** (e.g., `400 Bad Request`, `401 Unauthorized`, `500 Internal Server Error`).
  - **Providing descriptive error messages** in the response body to help clients understand the issue.
  - **Consistent error structure**, often including error codes, messages, and any necessary details.

**Example**:

```json
{
  "error": {
    "code": 400,
    "message": "Invalid request",
    "details": "The 'id' parameter is missing"
  }
}
```

---

### 19. **What is JSON Web Token (JWT), and how is it used in APIs?**

- **Answer**: **JWT** is a compact, URL-safe token format used for securely transmitting information between parties as a JSON object. It is commonly used for API authentication. The token contains claims, which are encoded and signed to ensure integrity and authenticity.

**Flow**:

1.  User logs in and receives a JWT from the server.
2.  The client includes the JWT in subsequent requests (usually in the `Authorization` header).
3.  The server verifies the JWT before processing the request.

**Example**:

```http
GET /profile
Authorization: Bearer <JWT Token>
```

---

### 20. **How do API versioning and backward compatibility work?**

- **Answer**: API versioning ensures that changes to the API (new features, breaking changes) don't disrupt clients using older versions. Common versioning strategies include:
  - **URL-based versioning**: `/api/v1/users`
  - **Header-based versioning**: `Accept: application/vnd.example.v1+json`
  - **Query parameter versioning**: `/users?version=1`

**Backward Compatibility**: When introducing changes, maintain backward compatibility by ensuring older clients can continue to function without issues. This might involve supporting multiple versions of the API or providing default behavior for legacy clients.

---

### 21. **What is API throttling, and why is it important?**

- **Answer**: **API throttling** limits the number of API requests a client can make within a certain time frame. It prevents overuse or abuse of the API, protecting the server from being overwhelmed and ensuring fair usage among clients.

**Example**: An API may limit a client to 1000 requests per hour. If the client exceeds this limit, the server might respond with a `429 Too Many Requests` status code and provide information on when the client can make more requests.

---

### 22. **What is GraphQL, and how does it differ from traditional REST APIs?**

- **Answer**: **GraphQL** is a query language for APIs that allows clients to specify exactly what data they need, avoiding over-fetching or under-fetching data. Unlike REST, where multiple endpoints may be required for complex queries, GraphQL provides a single endpoint with flexible queries.

**Example Query**:

```graphql
{
  user(id: 1) {
    name
    email
    posts {
      title
      comments {
        content
      }
    }
  }
}
```

This single query fetches the user, their posts, and comments in one request, whereas a REST API might require multiple endpoints to achieve the same.

---

### 23. **How do you secure APIs?**

- **Answer**: API security involves several strategies:
  - **Authentication and Authorization**: Implementing mechanisms like OAuth, JWT, or API keys to ensure only authorized users can access certain resources.
  - **Encryption**: Use HTTPS to encrypt data in transit.
  - **Input validation**: Sanitize and validate incoming data to prevent attacks like SQL injection.
  - **Rate limiting**: Prevent abuse by limiting the number of requests a client can make.
  - **CORS**: Restrict which domains can access the API.

---

### 24. **What are the benefits of using an API Gateway?**

- **Answer**: An API Gateway acts as a single entry point for API requests and provides several benefits:
  - **Routing**: Directs requests to the appropriate microservices or backend services.
  - **Security**: Centralizes authentication, authorization, and encryption.
  - **Rate Limiting**: Applies throttling and quota limits.
  - **Load Balancing**: Distributes requests across multiple instances to ensure high availability.

---

### 25. **How would you design a pagination system for an API?**

- **Answer**: To design a pagination system, divide the dataset into smaller pages and provide metadata like the total number of records and pages. Common approaches include:
  - **Limit-offset**: Clients specify the number of items per page and the starting point.
    ```
    GET /users?limit=10&offset=20
    ```
  - **Cursor-based pagination**: Clients are provided with a cursor (a unique identifier for a record), and subsequent requests use this cursor to fetch the next set of results.
    ```
    GET /users?cursor=eyJvZmZzZXQiOjEwfQ
    ```

---

### 26. **What is a microservices architecture, and how does it relate to APIs?**

- **Answer**: A **microservices architecture** breaks down a monolithic application into smaller, independent services that communicate with each other through APIs. Each microservice handles a specific function and can be developed, deployed, and scaled independently. APIs, often RESTful, serve as the communication layer between these services.

---

### 27. **How does caching work in APIs, and why is it important?**

- **Answer**: Caching improves API performance by storing responses for a defined period, allowing future requests to retrieve the cached data instead of reprocessing. Caching can be implemented at several levels:
  - **Client-side caching**: The client stores the response and reuses it for subsequent requests.
  - **Server-side caching**: The server caches frequently requested resources.
  - **Proxy caching**: A proxy server (like a CDN) caches responses to reduce load on the origin server.

**Example**: Using HTTP headers like `Cache-Control` to specify caching rules:

```
Cache-Control: public, max-age=3600
```

---

### 28. **What is the role of middleware in APIs?**

- **Answer**: Middleware functions in APIs are used to process requests before they reach the main application logic. They can be used for:
  - **Authentication and authorization**.
  - **Logging**.
  - **Rate limiting**.
  - **Input validation**.

Middleware functions can be applied to specific routes or globally for all API endpoints.
