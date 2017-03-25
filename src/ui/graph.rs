use std::collections::HashMap;

use petgraph::stable_graph::StableGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, DfsPostOrder};
use petgraph::Direction;
use petgraph::visit::Visitable;
use petgraph::stable_graph::WalkNeighbors;

use widget::{Widget, WidgetContainer};
use util::Point;
use resources::{resources, WidgetId};

type Graph = StableGraph<WidgetContainer, ()>;

pub struct WidgetGraph {
    pub graph: Graph,
    pub root_id: WidgetId,
    widget_map: HashMap<WidgetId, NodeIndex>,
}
impl WidgetGraph {
    pub fn new() -> Self {
        WidgetGraph {
            graph: StableGraph::new(),
            widget_map: HashMap::new(),
            root_id: resources().widget_id(),
        }
    }

    pub fn get_widget(&mut self, widget_id: WidgetId) -> Option<&mut Widget> {
        if let Some(node_index) = self.widget_map.get(&widget_id) {
            if let Some(widget_container) = self.graph.node_weight_mut(node_index.clone()) {
                return Some(&mut widget_container.widget)
            }
        }
        None
    }
    pub fn get_widget_container(&mut self, widget_id: WidgetId) -> Option<&mut WidgetContainer> {
        if let Some(node_index) = self.widget_map.get(&widget_id) {
            if let Some(widget_container) = self.graph.node_weight_mut(node_index.clone()) {
                return Some(widget_container)
            }
        }
        None
    }

    pub fn add_widget(&mut self,
                      widget: WidgetContainer,
                      parent_id: Option<WidgetId>)
                      -> NodeIndex
    {
        let id = widget.widget.id;
        let widget_index = self.graph.add_node(widget);
        self.widget_map.insert(id, widget_index);
        if let Some(parent_id) = parent_id {
            if let Some(parent_index) = self.find_widget(parent_id) {
                self.graph.add_edge(parent_index, widget_index, ());
            }
        }
        widget_index
    }

    pub fn remove_widget(&mut self, widget_id: WidgetId) -> Option<WidgetContainer> {
        if let Some(node_index) = self.find_widget(widget_id) {
            self.widget_map.remove(&widget_id);
            if let Some(widget) = self.graph.remove_node(node_index) {
                return Some(widget);
            }
        }
        None
    }
    fn find_widget(&self, widget_id: WidgetId) -> Option<NodeIndex> {
        self.widget_map.get(&widget_id).map(|index| *index)
    }
    fn root_index(&self) -> NodeIndex {
        self.find_widget(self.root_id).unwrap()
    }
    pub fn get_root(&mut self) -> &mut Widget {
        let root_id = self.root_id;
        self.get_widget(root_id).unwrap()
    }

    pub fn parent(&mut self, widget_id: WidgetId) -> Option<WidgetId> {
        let node_index = if let Some(node_index) = self.widget_map.get(&widget_id) {
            node_index.clone()
        } else {
            NodeIndex::end()
        };
        NeighborsWalker::new(&self.graph, node_index, Direction::Incoming).next(&self.graph)
    }
    pub fn children(&mut self, widget_id: WidgetId) -> NeighborsWalker {
        let node_index = if let Some(node_index) = self.widget_map.get(&widget_id) {
            node_index.clone()
        } else {
            NodeIndex::end()
        };
        NeighborsWalker::new(&self.graph, node_index, Direction::Outgoing)
    }
    pub fn widgets_under_cursor(&mut self, point: Point) -> CursorWidgetWalker {
        CursorWidgetWalker::new(point, &self.graph, self.root_index())
    }
    pub fn dfs(&mut self, widget_id: WidgetId) -> DfsWalker {
        let node_index = self.widget_map.get(&widget_id).unwrap();
        DfsWalker::new(&self.graph, node_index.clone())
    }
}

pub struct NeighborsWalker {
    neighbors: WalkNeighbors<u32>,
}
impl NeighborsWalker {
    fn new(graph: &Graph, node_index: NodeIndex, direction: Direction) -> Self {
        NeighborsWalker {
            neighbors: graph.neighbors_directed(node_index, direction).detach()
        }
    }
    pub fn next(&mut self, graph: &Graph) -> Option<WidgetId> {
        if let Some((_, node_index)) = self.neighbors.next(graph) {
            Some(graph[node_index].widget.id)
        } else {
            None
        }
    }
    pub fn collect(&mut self, graph: &Graph) -> Vec<WidgetId> {
        let mut ids = Vec::new();
        while let Some(id) = self.next(graph) {
            ids.push(id);
        }
        ids
    }
}

pub struct CursorWidgetWalker {
    point: Point,
    dfs: DfsPostOrder<NodeIndex, <Graph as Visitable>::Map>,
}
impl CursorWidgetWalker {
    fn new(point: Point, graph: &Graph, root_index: NodeIndex) -> Self {
        CursorWidgetWalker {
            point: point,
            dfs: DfsPostOrder::new(graph, root_index),
        }
    }
    pub fn next(&mut self, graph: &Graph) -> Option<WidgetId> {
        while let Some(node_index) = self.dfs.next(graph) {
            let ref widget = graph[node_index].widget;
            if widget.is_mouse_over(self.point) {
                return Some(widget.id);
            }
        }
        None
    }
}
pub struct DfsWalker {
    dfs: Dfs<NodeIndex, <Graph as Visitable>::Map>,
}
impl DfsWalker {
    fn new(graph: &Graph, root_index: NodeIndex) -> Self {
        DfsWalker {
            dfs: Dfs::new(graph, root_index),
        }
    }
    pub fn next(&mut self, graph: &Graph) -> Option<WidgetId> {
        if let Some(node_index) = self.dfs.next(graph) {
            Some(graph[node_index].widget.id)
        } else {
            None
        }
    }
}