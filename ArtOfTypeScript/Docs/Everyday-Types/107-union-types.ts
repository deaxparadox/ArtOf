const productInfo = function(
    obj: {
    price: number;
    item?: string|number 
}): void {
    console.log(obj.price)
    if (obj.item !== undefined) {
        console.log(obj.item)
    }
}

productInfo({price: 100})
productInfo({price: 80, item: "Item 1"})