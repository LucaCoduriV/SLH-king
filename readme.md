1. Mistakes found and changes made:

- Authentication: The original code did not implement any form of authentication for either students or teachers. I added a login system for both students and teachers, where they are prompted to enter their username and password before accessing any other functionality. For teachers, the username and password are checked against a predefined set of credentials stored in a secure manner. For students, their credentials are stored in a database and checked against it.

- Authorization: The original code did not implement any form of authorization, allowing any student to access the grades of any other student. I added an authorization system where only the student whose grades are being accessed can see their own grades, and only teachers can access the grades of any student.

- Logging: The original code did not implement any logging, making it difficult to track any security breaches or errors. I added logging to track all login attempts, access to grades, and any errors that occur.

- Encryption: The original code did not implement any encryption, leaving sensitive data such as student grades and teacher credentials vulnerable to being compromised. I added encryption for both the student grades and the teacher credentials, to protect them from unauthorized access.

- Error management: The original code did not handle errors in a consistent or secure manner, often panicking and revealing sensitive information. I added proper error handling and ensured that any errors that occur do not reveal sensitive information.

2. High-level report:
- Implemented a login system for both students and teachers, with proper authentication and authorization.

- Added logging to track all login attempts, access to grades, and any errors that occur.

- Implemented encryption for both student grades and teacher credentials to protect them from unauthorized access.

- Improved error handling to ensure that any errors that occur do not reveal sensitive information.

- Added input validation to ensure that only valid input is accepted, and output validation to ensure that any sensitive information is displayed only to authorized users.

3. The changes made should allow the application to continue to function as intended, allowing for the adding and viewing of grades, and separating the functionality for students and teachers.

4. The goal of the project is to secure the application and not to improve its UX. Nevertheless, if you wish to improve on this side, you can. Thus, there is no need to change for a more complex database system.