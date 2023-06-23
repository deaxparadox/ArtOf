import matplotlib.pyplot as plt
import numpy as np

def f(t):
    return np.exp(-t) * np.cos(2*np.pi*t)

t1 = np.arange(0.0, 5.0, 0.1)
t2 = np.arange(0.0, 5.0, 0.02)
t3 = np.arange(0.0, 5.0, 0.03)

plt.figure()

plt.subplot(221)
plt.title("Sample 1")
plt.plot(t1, f(t1), 'bo', t2, f(t2), 'k')

plt.subplot(222)
plt.title("Sample 2")
plt.plot(t2, np.cos(2*np.pi*t2), 'r--')


plt.subplot(223)
plt.title("Sample 3")
plt.plot(t3, np.sin(2*np.pi*t3), 'r--')


plt.subplot(224)
plt.title("Sample 3")
plt.plot(t3, np.tan(2*np.pi*t3), 'r--')

plt.show()