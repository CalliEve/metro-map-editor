# Editor Documentation

This document is a general documentation for the editor.
If anything is unclear or you want more detail on a certain function, please create an issue for me to address it.

Some terminology beforehand:

- Node: a coordinate on the grid
- Edge: a connection between two stations that one or more lines pass through.
- Degree-two station: a station that has exactly two edges connected to it with the same lines on both.
- Line-section: a continuously connected chain of stations and edges between two non-degree-two stations.

## Loading and saving

To edit an existing map, it needs to be loaded into the editor.
To do this, press the "upload file" button in the top-right corner of the screen.
The editor accepts graphml files and json files as long as they adhere a the custom format.
See the example files in the [existing_maps](../existing_maps) folder of this repository.

To download the map you have created or edited, press the "download map" button in the top-right corner of the screen.
This downloads the map as a JSON file for you.
Note that this removes any checkpoints on the map.

To save the map as an image, press the "To PNG" button.
This turns the current view of the map into a PNG image and downloads it for you.
Note that any checkpoints will be removed and no background (so also no grid) will be in the image.

## Moving around and other general tasks

To move around on the map, use the arrow keys.
You can also click on an empty spot of the canvas, keep the mouse button pressed and drag it around, to move the canvas as a whole.

To zoom in or out, use the scroll-wheel, or for more precise control use the + and - buttons in the bottom-right corner of the map.

Use CTRL-z to undo any changes you made up to 5 actions in the past.
These can then be redone using CTRL-shift-z.
You can also use the buttons in the bottom-left corner of the map.
These will also show if undo and redo is currently possible.

In the top-right corner of the map, there is an overlay button.
Using this, you can toggle to overlay the original map that was loaded in over the currently depicted map.

## Using the algorithm

To use the built-in algorithm to recalculate the map as it currently is, press the button in the top-right corner of the map.
This will reroute all edges between stations and (potentially) move all stations as needed to make the map into an octilinear grid map.
It will make all degree-two stations be equi-distance from their neighbors where possible.
All the while, it will be trying to minimize bends.
For more details, please see my master thesis that is associated with this editor.

To see the algorithm at work, press the smaller recalculate button under the big one.
This will show real-time updates as the algorithm is recalculating the map.
Note that these updates will have all degree-two stations removed, as that is the first step in map recalculation and they get added back in at the end.

While the algorithm is running, the "recalculate with real-time updates" button is replaced with an abort button.
Pressing this button will stop the recalculation and reset the map to before the recalculation was started.

### Partial recalculation

When at least two stations and an edge have been selected, the recalculate button is replaced with a partial recalculation button.
Pressing this button, will run the algorithm on only the part of the map that is currently selected.

Multiple stations and/or edges can be selected at once by holding shift and clicking on them.
Another way to select multiple is hold shift, click and hold on an empty spot of the canvas, and then drag the mouse to create a selection box.
Lastly, another method is to select a line-section all at once by double-clicking on an edge while holding shift.
These three methods can also be combined.

### Advanced settings

Click on the "advanced settings" button at the top of the screen to change the settings for the algorithm.
This allows for enabling debug output in the console, changing the amount of tries before giving up on trying to route the map, toggling the local search optimization and more.
It also allows for changing the grid size instead of zooming in, to have a different grid size for when a map is loaded in.

## Adding and moving stations

To add a new station to the map, click on the "add station" button on the left.
Then, click on the node on the canvas where you want to place it.

To move a station, click on the station and while continuing to hold the mouse clicked, drag the station to its new location.
When multiple stations have been selected, click and hold on one to drag and move all selected stations together.

Clicking on a station without moving it, brings up a box with information like the name of the station.
That name can also be edited in this box.

Lastly, to remove a station, click on the "remove station" button on the left, before clicking on the station to remove.
Hold shift while clicking on stations to remove, to remove multiple stations at once.

### Checkpoints

Checkpoints get removed when downloading a map or making the current view into a PNG.
This allows for making lines cross each other, or follow a route that the algorithm would otherwise change.

They are shown on the map as a diamond-shaped icon and can be added and removed similarly to stations with their own "add checkpoint" and "remove checkpoint" buttons.
To the algorithm and in other interactions they will also act exactly like normal stations.

## Adding and moving lines

To add a line, click on the "add line" button on the left, before clicking on a station.
This will add the station to the map and adds the station as the first (and currently only) station on the line.

Click on a node adjacent to a station that the line ends on (aka, it is connected to the station by only one edge), before dragging the line onto another station, to extend the line to that station.
Click on a node that a line travels through between two stations, then drag the line onto another station, to insert that station between the first two.

Clicking on an edge without moving it, brings up a box with information like the name of all the lines going through that edge.
It also allows for editing the names and colors of all those lines.

Click on the "remove line" button on the left, before clicking on a line, to remove it from the map altogether.

## locking

Locking a station or edge prevents the algorithm from changing its position or routing.
If an edge has been locked, its start and end stations will not be moved either.

Locking works similarly to deleting a station: click on "lock" or "unlock" button on the left, before clicking on the station or edge to lock/unlock.
Hold shift while clicking on the targets to lock/unlock multiple at once.

## Straightening lines

When you have selected at least 2 stations and an edge between them, you can use the "straighten selected" button.
This will move the start and end stations of the selected chain of edges as needed and straighten the line between them.
The stations in-between will, in addition to be moved vertically as needed, also be moved as needed to ensure they are equi-distanced from each other (unless they are connected by edges not in the selection).
Any length of a chain of edges and stations can be selected and straightened at once, as long as the chain is continuous.
