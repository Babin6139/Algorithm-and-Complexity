import unittest
import pandas as pd
import networkx as nx
from graph_task1 import average_degree,density,diameter,clusteringCoefficient,number_edge_and_vertices


class Graph_Test(unittest.TestCase):

    def setUp(self):
        self.header_list = ["a", "b", "w"]
        self.E = pd.read_csv('./dataset/aves-barn-swallow-contact-network.edges', sep = " ", header = None, names = self.header_list)
        self.G = nx.from_pandas_edgelist(self.E, "a", "b", ["w"])

        self.num_nodes = nx.number_of_nodes(self.G)
        self.num_edges = nx.number_of_edges(self.G)

    def test_avg_degree(self):
        degree = 0
        for node in nx.nodes(self.G):
            degree += self.G.degree(node)
        avg = degree/self.num_nodes

        self.assertEqual(average_degree(self.G), int(avg))

    def test_Density(self):
        self.assertEqual(density(self.G), round(nx.density(self.G), 5))

    def test_Diameter(self):
        self.assertEqual(diameter(self.G), nx.diameter(self.G))
    
    def test_ClusteringCoefficient(self):
        self.assertEqual(clusteringCoefficient(self.G), round(nx.average_clustering(self.G), 5))


if __name__ == "__main__":
    unittest.main()