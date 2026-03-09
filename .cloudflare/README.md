We have three Cloudflare workers that let us serve some assets of this repo
from Cloudflare.

- `open-source-website-assets` is used for static open-source assets such as install helpers
- `docs-proxy` is used for `https://superzet.dev/docs`
- `release-assets` is used for `https://superzet.dev/releases`

On push to `main`, these workers and the docs build are uploaded to Cloudflare.

### Deployment

These functions are deployed on push to main by the `deploy_cloudflare.yml` workflow. Worker rules in Cloudflare should route `superzet.dev/docs*` and `superzet.dev/releases*` to the corresponding workers.

For the release worker, configure an optional `GITHUB_RELEASES_TOKEN` secret in Cloudflare if you want higher GitHub API rate limits for update checks.

### Testing

You can use [wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update) to test these workers locally, or to deploy custom versions.
