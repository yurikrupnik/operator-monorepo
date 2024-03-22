## Writing Your Docs
### Writing Your Docs

```bash
kind create cluster --name kind-docs --config kind-config.yaml
```

```rust
fn main() {
    println!("Hello, world!");
}
```

```python
print("Hello, world!")
```

```javascript
console.log("Hello, world!")
```

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod
  namespace: a-team
spec:
    containers:
    - name: my-container
      image: nginx
```