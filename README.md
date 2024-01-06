# DAGAL
**D**anny's **A**wesome **G**raphics **A**bstraction **L**ayer

---
DAGAL abstracts Vulkan and is mainly only used by me.

DAGAL is split into two parts:
- **Abstraction** - Contains only abstractions for Vulkan, but nothing more
- **Framework** - Contains utility for Vulkan such as render graphs, lifetime management, etc.


## Minimum device requirements
- - -
We expect this to be used on devices which support at the minimum:
- Dynamic Rendering
- Descriptor Indexing

tl;dr if your GPU supports Vulkan 1.3, you're already supported.


## To-dos (we will never finish them)
- - -
- [ ] Render Graph
- [ ] Lifetime management using timeline semaphores
- [ ] egui integration
- [ ] GPU Resource Table


## Special thanks
- - -
Special thanks to [phobos-rs](https://github.com/NotAPenguin0/phobos-rs) and NotAPenguin for providing a significant
base layer for me to work from.