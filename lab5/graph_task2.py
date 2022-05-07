import pandas as pd
import networkx as nx
import matplotlib.pyplot as plt

from graph_task1 import (average_degree, clusteringCoefficient, density, diameter, number_edge_and_vertices)

def plot_degree_distribution(G, label):
    frequencyList = nx.degree_histogram(G)
    V=nx.number_of_nodes(G)
    probDegreeList = [x/V for x in frequencyList]
    degreeList = [*range(0, len(frequencyList))]
    plt.figure("Degree Distribution")
    plt.xlabel("k")
    plt.ylabel("P(k)")
    plt.title(label)
    plt.plot(degreeList, probDegreeList)
    plt.show()



if __name__=="__main__":
    datasets=["3elt","176bit","bio-CE-CX","socfb-Lehigh96","web-spam"]
    for dataset in datasets:
        header_list=["a","b","w"]
        path="./dataset/"+dataset+".edges"
        E=pd.read_csv(path,sep=" ", header=None, names=header_list)
        G=nx.from_pandas_edgelist(E,"a","b",["w"])
        print(f"Number of vertices and edges are {number_edge_and_vertices(G)}")
        print(f"Average Degree: {average_degree(G)}")
        print(f"Density: {density(G)}")
        print(f"Diameter: {diameter(G)}")
        print(f"Clustering coefficient: {clusteringCoefficient(G)}")
        plot_degree_distribution(G,dataset)
