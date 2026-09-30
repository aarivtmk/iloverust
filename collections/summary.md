```text
| Operation                  |    Array |                Vec |           VecDeque |                                          LinkedList |           HashMap |
| -------------------------- | -------: | -----------------: | -----------------: | --------------------------------------------------: | ----------------: |
| **Access by index** `x[i]` | **O(1)** |           **O(1)** |           **O(1)** |                                            **O(n)** |               N/A |
| **Search by value/key**    |     O(n) |               O(n) |               O(n) |                                                O(n) |      **O(1) avg** |
| **Push front**             |     N/A* |           **O(n)** |           **O(1)** |                                            **O(1)** |               N/A |
| **Push back**              |     N/A* | **O(1) amortized** | **O(1) amortized** |                                            **O(1)** |               N/A |
| **Pop front**              |     N/A* |           **O(n)** |           **O(1)** |                                            **O(1)** |               N/A |
| **Pop back**               |     N/A* |           **O(1)** |           **O(1)** |                                            **O(1)** |               N/A |
| **Insert middle**          |     N/A* |           **O(n)** |           **O(n)** | **O(n)** to find position; **O(1)** once node known |                   |
| **Remove middle**          |     N/A* |           **O(n)** |           **O(n)** | **O(n)** to find position; **O(1)** once node known |                   |
| **Resize/grow**            |    Fixed |  O(n) occasionally |  O(n) occasionally |                                No contiguous resize | O(n) occasionally |
| **Memory**                 |     O(n) |               O(n) |               O(n) |                                                O(n) |              O(n) |

```
