# Sidewalk add-on checkout

A responsive checkout prototype for a concrete project. Customers can select
optional services, enter sidewalk length and width, see the calculated square
footage, and review live pricing before confirmation.

## Run locally

From the repository root:

```bash
python3 -m http.server 4173 --directory examples/sidewalk-checkout
```

Then open <http://localhost:4173>.

This is a front-end demonstration only. The confirmation action does not send
or persist customer information.