# Error Checklist

Whenever writing a new function, ask the following questions.

---

## 1. Can this operation fail?

Examples:

- Parsing
- Database
- File I/O
- Network
- Serialization
- Permissions
- Transactions

If yes:

→ Return `Result`.

---

## 2. Is this caused by external input?

Examples:

- User input
- Database unavailable
- Missing file
- Invalid UUID

If yes:

→ Return an error.

---

## 3. Is this a business rule?

Examples:

- Negative balance
- Empty account name
- Budget exceeded
- Duplicate account

If yes:

→ Return a domain error.

---

## 4. Is this my bug?

Examples:

- Impossible state
- Broken invariant
- Unreachable code

If yes:

→ Consider `panic!()` or `debug_assert!()`.

---

## 5. Can this layer recover?

Yes:

→ Handle it.

No:

→ Propagate it using `?`.

---

## 6. Can I provide a better abstraction?

If yes:

→ Translate the error.

Otherwise:

→ Propagate it unchanged.

---

## Remember

Every layer should either:

- Handle
- Translate
- Propagate

Never silently ignore an error.
