from itertools import combinations
import pandas as pd
import networkx as nx
import matplotlib.pyplot as plt


def number_edge_and_vertices(G):
    return (nx.number_of_nodes(G),nx.number_of_edges(G))

def average_degree(G):
    totalDegree = 0
    V=nx.number_of_nodes(G)
    for node in nx.nodes(G):
        count = 0
        for neighbor in nx.neighbors(G, node):
            count += 1
        totalDegree += count

    avgDegree = totalDegree/V
    return(int(avgDegree))

def density(G):
    (V,E)=number_edge_and_vertices(G)
    if V > 1:
        return(round((2*E)/(V*(V - 1)), 5))
    else:
        return(0)

def diameter(G):
    try:
        return nx.diameter(G)
    except:
        return "Diameter is infinite since disconnected graph"
    
def clusteringCoefficient(G):
    clusterCoeff = 0
    V=nx.number_of_nodes(G)
    for node in nx.nodes(G):
        Ki, Ei = 0, 0
        for i in G.neighbors(node):
            Ki += 1

        for v1, v2 in combinations(G.neighbors(node), 2):
            if G.has_edge(v1, v2):
                Ei += 1
        
        if Ki > 1:
            clusterCoeff += (2*Ei)/(Ki*(Ki - 1))
    
    return(round(clusterCoeff/V, 5))

if __name__=="__main__":
    header_list=["a","b","w"]
    E=pd.read_csv("./dataset/aves-barn-swallow-contact-network.edges",sep=" ", header=None, names=header_list)
    G=nx.from_pandas_edgelist(E,"a","b",["w"])
    print(f"Number of vertices and edges are {number_edge_and_vertices(G)}")
    print(f"Average Degree: {average_degree(G)}")
    print(f"Density: {density(G)}")
    print(f"Diameter: {diameter(G)}")
    print(f"Clustering coefficient: {clusteringCoefficient(G)}")
    plt.title("aves-barn-swallow-network")
    nx.draw(G)
    plt.show()

