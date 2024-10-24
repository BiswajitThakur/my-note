# Mongoose

**Mongoose** is an Object Data Modeling (ODM) library for MongoDB and Node.js. It provides a schema-based solution to model your data, and it manages relationships between data, allows for schema validation, and handles queries. Mongoose simplifies interactions with MongoDB, making it easier to work with the database through its flexible, schema-based models.

---

### Key Features of Mongoose:

1. **Schema-based modeling**: Mongoose allows you to define data models with schemas, which provide a clear structure for the data.
2. **Validation**: Mongoose has built-in validation for schema fields, which helps ensure that only valid data is saved.
3. **Middleware (pre/post hooks)**: You can define pre- and post-save hooks for operations, which allow for running custom logic before or after a certain operation.
4. **Query building**: It simplifies MongoDB queries by providing a fluent query-building interface.
5. **Plugins**: Mongoose has a plugin system, enabling you to extend functionality (e.g., add timestamps or pagination).

---

### Installation

You can install Mongoose via npm:

```bash
npm install mongoose
```

### Basic Usage Example

#### Step 1: Connect to MongoDB

```javascript
const mongoose = require("mongoose");

// Connect to MongoDB
mongoose.connect("mongodb://localhost/mydatabase", {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

const db = mongoose.connection;

db.on("error", console.error.bind(console, "connection error:"));
db.once("open", () => {
  console.log("Connected to MongoDB");
});
```

#### Step 2: Define a Schema and Model

```javascript
// Define a schema for a "User"
const userSchema = new mongoose.Schema({
  name: String,
  email: String,
  age: Number,
});

// Create a model based on the schema
const User = mongoose.model("User", userSchema);
```

#### Step 3: Create and Save a Document

```javascript
// Create a new user instance
const newUser = new User({
  name: "Alice",
  email: "alice@example.com",
  age: 25,
});

// Save the user to the database
newUser.save((err, user) => {
  if (err) return console.error(err);
  console.log("User saved:", user);
});
```

#### Step 4: Querying Data

```javascript
// Find all users
User.find({}, (err, users) => {
  if (err) return console.error(err);
  console.log("Users:", users);
});

// Find a user by name
User.findOne({ name: "Alice" }, (err, user) => {
  if (err) return console.error(err);
  console.log("User found:", user);
});
```

## Mongo and Express Example

### Steps:

1. **Install dependencies**
2. **Connect to MongoDB using Mongoose**
3. **Create the Express app**
4. **Define routes for CRUD operations**
5. **Start the server**

---

### 1. Install Dependencies

First, you’ll need to install the required npm packages.

```bash
npm install express mongoose
```

### 2. Setup Mongoose and Connect to MongoDB

Create a `server.js` file.

```javascript
const express = require("express");
const mongoose = require("mongoose");

// Connect to MongoDB
mongoose.connect("mongodb://localhost:27017/mydatabase", {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

const db = mongoose.connection;

db.on("error", console.error.bind(console, "MongoDB connection error:"));
db.once("open", () => {
  console.log("Connected to MongoDB");
});

const app = express();

// Middleware to parse incoming JSON requests
app.use(express.json());

// Start the server
const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
```

### 3. Create a Mongoose Schema and Model

In the same `server.js` file, create a Mongoose schema and model for a `User`.

```javascript
// Define the User schema
const userSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true,
  },
  email: {
    type: String,
    required: true,
    unique: true,
  },
  age: Number,
});

// Create the User model
const User = mongoose.model("User", userSchema);
```

### 4. Create CRUD Routes

Now, define routes in Express to perform basic CRUD operations for the `User` model.

#### Create a new user (POST):

```javascript
// POST: Create a new user
app.post("/users", async (req, res) => {
  try {
    const user = new User(req.body); // Create a new user with the request body
    await user.save(); // Save the user to the database
    res.status(201).send(user); // Return the created user
  } catch (err) {
    res.status(400).send(err); // Return an error if user creation fails
  }
});
```

#### Get all users (GET):

```javascript
// GET: Get all users
app.get("/users", async (req, res) => {
  try {
    const users = await User.find(); // Find all users in the database
    res.status(200).send(users); // Return the users
  } catch (err) {
    res.status(500).send(err); // Return an error if fetching fails
  }
});
```

#### Get a user by ID (GET):

```javascript
// GET: Get a user by ID
app.get("/users/:id", async (req, res) => {
  try {
    const user = await User.findById(req.params.id); // Find the user by ID
    if (!user) return res.status(404).send(); // Return 404 if user not found
    res.status(200).send(user); // Return the user
  } catch (err) {
    res.status(500).send(err); // Return an error if fetching fails
  }
});
```

#### Update a user by ID (PUT):

```javascript
// PUT: Update a user by ID
app.put("/users/:id", async (req, res) => {
  try {
    const user = await User.findByIdAndUpdate(req.params.id, req.body, {
      new: true, // Return the updated document
      runValidators: true, // Run schema validators
    });
    if (!user) return res.status(404).send(); // Return 404 if user not found
    res.status(200).send(user); // Return the updated user
  } catch (err) {
    res.status(400).send(err); // Return an error if updating fails
  }
});
```

#### Delete a user by ID (DELETE):

```javascript
// DELETE: Delete a user by ID
app.delete("/users/:id", async (req, res) => {
  try {
    const user = await User.findByIdAndDelete(req.params.id); // Find and delete the user by ID
    if (!user) return res.status(404).send(); // Return 404 if user not found
    res.status(200).send(user); // Return the deleted user
  } catch (err) {
    res.status(500).send(err); // Return an error if deletion fails
  }
});
```

### 5. Start the Server

Now you can start the server with the command:

```bash
node server.js
```

---

### Example Usage

Once the server is running, you can interact with it using an API client like Postman or `curl`.

1. **Create a User** (POST):

   ```bash
   curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d '{"name": "John", "email": "john@example.com", "age": 30}'
   ```

2. **Get All Users** (GET):

   ```bash
   curl http://localhost:3000/users
   ```

3. **Get a User by ID** (GET):

   ```bash
   curl http://localhost:3000/users/<user_id>
   ```

4. **Update a User by ID** (PUT):

   ```bash
   curl -X PUT http://localhost:3000/users/<user_id> -H "Content-Type: application/json" -d '{"name": "John Updated", "email": "john@example.com", "age": 31}'
   ```

5. **Delete a User by ID** (DELETE):
   ```bash
   curl -X DELETE http://localhost:3000/users/<user_id>
   ```

---

### Questions and Answers

#### Q1: **What is Mongoose in Node.js?**

**Answer**: Mongoose is an Object Data Modeling (ODM) library for MongoDB and Node.js. It provides a schema-based structure for defining the data model, validation, and querying, making it easier to interact with MongoDB.

---

#### Q2: **How do you define a schema in Mongoose?**

**Answer**: A schema in Mongoose defines the structure of the documents within a collection. You can define a schema by using `mongoose.Schema`.

Example:

```javascript
const userSchema = new mongoose.Schema({
  name: String,
  email: String,
  age: Number,
});
```

---

#### Q3: **What is a model in Mongoose?**

**Answer**: A model in Mongoose is a compiled version of the schema. It represents a collection in the database and allows you to interact with documents in that collection (e.g., perform CRUD operations).

Example:

```javascript
const User = mongoose.model("User", userSchema);
```

---

#### Q4: **How do you perform data validation in Mongoose?**

**Answer**: Mongoose provides built-in validation through schemas. You can specify validation rules (e.g., required fields, data types) directly in the schema definition.

Example:

```javascript
const userSchema = new mongoose.Schema({
  name: { type: String, required: true },
  email: { type: String, required: true },
  age: { type: Number, min: 18, max: 100 },
});
```

---

#### Q5: **How do you connect to MongoDB using Mongoose?**

**Answer**: To connect to MongoDB using Mongoose, you use the `mongoose.connect()` function, passing in the MongoDB connection URI and connection options.

Example:

```javascript
mongoose.connect("mongodb://localhost/mydatabase", {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});
```

---

#### Q6: **What is a pre-hook in Mongoose, and how is it used?**

**Answer**: A pre-hook (or pre-middleware) in Mongoose is a function that runs before a specific action, such as `save`, `find`, or `validate`. It can be used to execute some logic before saving a document.

Example:

```javascript
userSchema.pre("save", function (next) {
  console.log("User is about to be saved");
  next();
});
```

---

#### Q7: **What is the difference between `find()` and `findOne()` in Mongoose?**

**Answer**:

- `find()` returns all documents that match the query as an array.
- `findOne()` returns the first document that matches the query.

Example:

```javascript
// Find all users
User.find({}, (err, users) => {
  console.log(users);
});

// Find one user by name
User.findOne({ name: "Alice" }, (err, user) => {
  console.log(user);
});
```

---

#### Q8: **How do you update a document in Mongoose?**

**Answer**: You can update a document using `updateOne()`, `updateMany()`, or `findByIdAndUpdate()` methods.

Example:

```javascript
// Update a user's email
User.updateOne(
  { name: "Alice" },
  { email: "newemail@example.com" },
  (err, res) => {
    if (err) console.error(err);
    console.log("User updated:", res);
  },
);
```

---

#### Q9: **How do you delete a document in Mongoose?**

**Answer**: You can delete a document using `deleteOne()`, `deleteMany()`, or `findByIdAndDelete()`.

Example:

```javascript
// Delete a user by name
User.deleteOne({ name: "Alice" }, (err) => {
  if (err) console.error(err);
  console.log("User deleted");
});
```

---

#### Q10: **What are virtuals in Mongoose?**

**Answer**: Virtuals are properties that are not stored in MongoDB but are computed on the fly when you access them. They are useful for deriving data based on existing fields.

Example:

```javascript
userSchema.virtual("fullName").get(function () {
  return `${this.firstName} ${this.lastName}`;
});
```

---

### More Questions and Answers on Mongoose

#### Q11: **What is the purpose of Mongoose plugins?**

**Answer**: Mongoose plugins are reusable pieces of code that can be added to schemas to extend their functionality. For example, you could add a timestamp plugin to automatically handle `createdAt` and `updatedAt` fields for all models.

---

#### Q12: **What is the purpose of `populate()` in Mongoose?**

**Answer**: `populate()` is used to replace a reference field in a document with the actual document it references. It’s useful for resolving relationships between collections (e.g., a user referencing posts).

Example:

```javascript
User.findOne({ name: "Alice" })
  .populate("posts")
  .exec((err, user) => {
    console.log(user.posts);
  });
```

---

#### Q13: **What is `lean()` in Mongoose, and why is it used?**

**Answer**: The `lean()` method returns plain JavaScript objects instead of Mongoose documents. It is used to improve performance when you don't need full Mongoose functionality (like methods, virtuals, etc.).

Example:

```javascript
User.find({})
  .lean()
  .exec((err, users) => {
    console.log(users);
  });
```

---

#### Q14: **How can you add indexes in Mongoose?**

**Answer**: Indexes can be defined at the schema level to improve query performance. You can define them by setting the `index` property in the schema.

Example:

```javascript
const userSchema = new mongoose.Schema({
  email: { type: String, unique: true },
});
```

---

#### Q15: **How does Mongoose handle schema relationships?**

**Answer**: Mongoose handles relationships using references (`ref`) and the `populate()` method. You can define a reference between documents by setting a field with the `ref` option, which points to another collection.

Example:

```javascript
const postSchema = new mongoose.Schema({
  title: String,
  author: { type: mongoose.Schema.Types.ObjectId, ref: "User" },
});
```

---

### Conclusion

Mongoose simplifies working with MongoDB by providing a structured, schema-based approach for defining data models. It also includes features like validation, middleware, virtuals, and query helpers, which make it a powerful tool for Node.js developers working with MongoDB.
