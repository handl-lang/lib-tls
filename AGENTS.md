# lib-tls

The `tls` **port contract** — step-based TLS surface implemented by
`tls-libssl`. Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the contract |
| `pkg.hnl` | package manifest — `modules: ["tls"]` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | the gate / bupha entry |

## Contract

- Step-based TLS surface (nonblocking primitives a scheduler can
  park on); the implementation is `lib-tls-libssl`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
