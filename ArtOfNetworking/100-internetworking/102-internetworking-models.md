# Internetworking Models

- *Open Systems Interconnection (OSI) reference model* was created by the International Orgaization for Standardization (ISO) to help vendors create interoperable network devices and software in the form of protocols so that different vendor network could work in peaceable accord with each other.

- OSI model is the primary architectural model for networks. It describes how data network information are communicated from an application on one computer through the network media to an application on another computer. The OSI model breaks this approach into layers.

### The Layered Approach

Understand that a *reference model* is a conceptual blueprint of how communications should take place. It addresses all the processes required for effective communication and divides
them into logical groupings called *layers*. When a communication system is designed in this manner, it’s known as a hierarchical or *layered architecture*.

----------

Models happen to be really important to software developers too. They often use a reference model to understand computer communication processes so they can determine which functions should be accomplished on a given layer. This means that if  someone is creating a protocol for a certain layer, they only need to be concerned with their target layer’s function. Software that maps to another layer’s protocols and is specifically designed
to be deployed there will handle additional functions. The technical term for this idea is *binding*. The communication processes that are related to each other are bound, or grouped
together, at a particular layer.


### Advantages of Reference Models

Here's a list oa some of the more important benefits of using the OSI layered model:

- It divides the network communication process into smaller and simpler components, facilitating component development, design, and troubleshooting.
- It allows mulitple-vecdor development through the standardization of network components.
- It encourages industry standardization by clearly defining what functions occurs at each layer of the model.
- It allows variour types of network hardware and software to communicate.
- It prevents changes in one layer from affecting other layers to expedite development.

